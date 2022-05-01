use crate::battle::{
    builder::{BattleBuilder, RandomOption},
    model::CharaConfig,
    rpg_core::PlayMode,
};
use crate::database::{
    playdata::Entity as PlaydataEntity,
    save::{Entity as UserDataEntity, Model},
};
use crate::setting::{
    i18n::i18n_text,
    setup::{config_parse_toml, Languages},
};
use anyhow::Context;
use once_cell::sync::Lazy;
use sea_orm::EntityTrait;
use serenity::client;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::prelude::ChannelId;
use serenity::model::user::User;
use std::time::Duration;

pub static BATTLE_REACTIONS: Lazy<Vec<ReactionType>> = Lazy::new(|| {
    let mut vec = Vec::new();
    vec.push(ReactionType::Unicode(BATTLE_PLAY.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_GUARD.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_ITEM.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_SAVE.to_string()));
    vec
});

pub static YES_NO_REACTIONS: Lazy<Vec<ReactionType>> = Lazy::new(|| {
    let mut vec = Vec::new();
    vec.push(ReactionType::Unicode("⭕".to_string()));
    vec.push(ReactionType::Unicode("❌".to_string()));
    vec
});

const BATTLE_PLAY: &str = "⚔";
const BATTLE_ITEM: &str = "💊";
const BATTLE_SAVE: &str = "✒️";
const BATTLE_GUARD: &str = "\u{1F6E1}";

pub async fn play(
    ctx: client::Context,
    channel_id: ChannelId,
    user: User,
    postgres_connect: sea_orm::DatabaseConnection,
) -> CommandResult {
    if !user.bot {
        let userdata = match UserDataEntity::find_by_id(user.id.to_string())
            .one(&postgres_connect)
            .await
            .map_err(|e| anyhow::anyhow!(e))?
        {
            Some(ud) => ud,
            None => {
                let model = Model {
                    exp: 1,
                    level: 1,
                    player: "Reimu".to_string(),
                    user_id: user.id.to_string(),
                    battle_uuid: None,
                };
                model.save(&postgres_connect).await;
                model
            }
        };

        let playdata = {
            match userdata.battle_uuid {
                Some(r) => PlaydataEntity::find_by_id(r)
                    .one(&postgres_connect)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?,
                None => None,
            }
        };

        let mut battle = match playdata {
            Some(d) => {
                let mut builder: BattleBuilder = d.into();
                builder.player_status_setting(userdata.level as i16);
                builder.enemy_status_setting(userdata.level as i16);
                builder.build()
            }
            None => {
                let mut init =
                    BattleBuilder::new(PlayMode::Simple, Some(userdata.clone().into()), None, None);

                init.enemy_random(RandomOption::default()).await;
                init.player_status_setting(1).enemy_status_setting(1);

                init.build()
            }
        };

        channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(format!(
                        "{}{}",
                        &battle.enemy().charabase.name,
                        i18n_text(Languages::Japanese).game_message.appear_enemy
                    ))
                    .description(if &battle.elapesd_turns() != &0 {
                        format!("{}ターン目です", &battle.elapesd_turns())
                    } else {
                        format!("最初からです")
                    })
                })
            })
            .await?;

        loop {
            if battle.turn() != battle.enemy() || battle.turn() == battle.player() {
                let operation_embed =
                    operation_enemy(&ctx, channel_id, BATTLE_REACTIONS.to_vec()).await?;
                if let Some(reaction) = &operation_embed
                    .await_reaction(&ctx)
                    .timeout(Duration::from_secs(
                        config_parse_toml().await.timeout_duration().unwrap_or(10),
                    ))
                    .author_id(user.id.0)
                    .await
                {
                    let emoji = &reaction.as_inner_ref().emoji;
                    match emoji.as_data().as_str() {
                        BATTLE_PLAY => {
                            let result = battle.result_battle().await;
                            if result.enemy().charabase.hp > 0 {
                                channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            let enemy = result.enemy();
                                            e.title(format!("敵ののこりhp{}", enemy.charabase.hp))
                                                .description(&enemy.charabase.name)
                                        })
                                    })
                                    .await
                                    .context("埋め込みの作成に失敗しました")?;
                            } else if result.enemy().charabase.hp <= 0 {
                                channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            e.title(format!(
                                                "{}を倒した",
                                                result.enemy().charabase.name
                                            ))
                                        })
                                    })
                                    .await
                                    .context("埋め込みの作成に失敗しました")?;
                                battle.reset_turn();
                                let user_exp =
                                    userdata.exp as f64 + battle.enemy().meta.get_exp as f64;
                                let player_level = battle.calculate_player_level(user_exp);
                                Model {
                                    user_id: user.id.0.to_string(),
                                    exp: user_exp as i64,
                                    level: player_level as i64,
                                    player: userdata.player.clone(),
                                    battle_uuid: Some(battle.uuid()),
                                }
                                .save(&postgres_connect)
                                .await;
                                break;
                            } else {
                                break;
                            }
                        }
                        BATTLE_GUARD => {
                            let result = battle.result_guard().await;
                            channel_id
                                .send_message(&ctx.http, |f| {
                                    f.embed(|e| {
                                        e.title(format!(
                                            "{}は防御した",
                                            &result.player().charabase.name
                                        ))
                                    })
                                })
                                .await?;
                        }
                        BATTLE_SAVE => {
                            let player_level = battle.calculate_player_level(
                                userdata.exp as f64 + battle.enemy().meta.get_exp as f64,
                            );
                            Model {
                                user_id: user.id.0.to_string(),
                                exp: userdata.exp as i64 + battle.enemy().meta.get_exp as i64,
                                level: player_level as i64,
                                player: userdata.player.clone(),
                                battle_uuid: Some(battle.uuid()),
                            }
                            .save(&postgres_connect)
                            .await;

                            let question = channel_id
                                .send_message(&ctx.http, |f| {
                                    f.embed(|e| {
                                        e.title("thrpgを続けますか？").description(
                                            "セーブされているので続きをプレイすることも可能です",
                                        )
                                    })
                                    .reactions(YES_NO_REACTIONS.to_vec())
                                })
                                .await
                                .context("埋め込みの作成に失敗しました")?;

                            if let Some(reaction) = &question
                                .await_reaction(&ctx)
                                .timeout(Duration::from_secs(
                                    config_parse_toml().await.timeout_duration().unwrap_or(10),
                                ))
                                .author_id(user.id)
                                .await
                            {
                                let emoji = &reaction.as_inner_ref().emoji;
                                match emoji.as_data().as_str() {
                                    "❌" => {
                                        break;
                                    }
                                    "⭕" => {
                                        channel_id
                                            .send_message(&ctx.http, |f| {
                                                f.embed(|e| e.title("thrpgを続けます"))
                                            })
                                            .await
                                            .context("埋め込みの作成に失敗しました")?;
                                    }
                                    _ => {
                                        error_embed_message(
                                            &ctx,
                                            channel_id,
                                            "正しい反応を選んで下さい",
                                        )
                                        .await?;
                                    }
                                }
                            }
                        }
                        _ => break,
                    }
                }
            } else if battle.elapesd_turns() == 0 && battle.turn() == battle.enemy() {
                let operation_embed =
                    operation_enemy(&ctx, channel_id, BATTLE_REACTIONS.to_vec()).await?;
                battle.add_turn();
                if let Some(reaction) = &operation_embed
                    .await_reaction(&ctx)
                    .timeout(Duration::from_secs(
                        config_parse_toml().await.timeout_duration().unwrap_or(10),
                    ))
                    .author_id(user.id)
                    .await
                {
                    let emoji = &reaction.as_inner_ref().emoji;
                    match emoji.as_data().as_str() {
                        BATTLE_PLAY => {
                            let result = battle.result_battle().await;
                            if result.enemy().charabase.hp > 0 {
                                channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            let enemy = result.enemy();
                                            e.title(format!("敵ののこりhp{}", enemy.charabase.hp))
                                                .description(&enemy.charabase.name)
                                        })
                                    })
                                    .await
                                    .context("埋め込みの作成に失敗しました")?;
                            } else if result.enemy().charabase.hp <= 0 {
                                channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            e.title(format!(
                                                "{}を倒した",
                                                result.enemy().charabase.name
                                            ))
                                        })
                                    })
                                    .await
                                    .context("埋め込みの作成に失敗しました")?;
                                battle.reset_turn();
                                break;
                            } else {
                                break;
                            }
                        }
                        BATTLE_GUARD => {
                            let result = battle.result_guard().await;
                            channel_id
                                .send_message(&ctx.http, |f| {
                                    f.embed(|e| {
                                        e.title(format!(
                                            "{}は防御した",
                                            &result.player().charabase.name
                                        ))
                                    })
                                })
                                .await?;
                        }
                        BATTLE_SAVE => {
                            let player_level = battle.calculate_player_level(
                                userdata.exp as f64 + battle.enemy().meta.get_exp as f64,
                            );
                            Model {
                                user_id: user.id.0.to_string(),
                                exp: userdata.exp as i64 + battle.enemy().meta.get_exp as i64,
                                level: player_level as i64,
                                player: userdata.player.clone(),
                                battle_uuid: Some(battle.uuid()),
                            }
                            .save(&postgres_connect)
                            .await;

                            let question = channel_id
                                .send_message(&ctx.http, |f| {
                                    f.embed(|e| {
                                        e.title("thrpgを続けますか？").description(
                                            "セーブされているので続きをプレイすることも可能です",
                                        )
                                    })
                                    .reactions(YES_NO_REACTIONS.to_vec())
                                })
                                .await
                                .context("埋め込みの作成に失敗しました")?;

                            if let Some(reaction) = &question
                                .await_reaction(&ctx)
                                .timeout(Duration::from_secs(
                                    config_parse_toml().await.timeout_duration().unwrap_or(10),
                                ))
                                .author_id(user.id)
                                .await
                            {
                                let emoji = &reaction.as_inner_ref().emoji;
                                match emoji.as_data().as_str() {
                                    "❌" => {
                                        break;
                                    }
                                    "⭕" => {
                                        channel_id
                                            .send_message(&ctx.http, |f| {
                                                f.embed(|e| e.title("thrpgを続けます"))
                                            })
                                            .await
                                            .context("埋め込みの作成に失敗しました")?;
                                    }
                                    _ => {
                                        error_embed_message(
                                            &ctx,
                                            channel_id,
                                            "正しい反応を選んで下さい",
                                        )
                                        .await?;
                                    }
                                }
                            }
                        }
                        _ => break,
                    }
                }
            } else {
                let result = battle.result_battle().await;
                if result.player().charabase.hp > 0 {
                    channel_id
                        .send_message(&ctx.http, |f| {
                            f.embed(|e| {
                                let player = result.player();
                                e.title(format!("味方ののこりhp{}", player.charabase.hp))
                                    .description(&player.charabase.name)
                            })
                        })
                        .await
                        .context("埋め込みの作成に失敗しました")?;
                } else if result.player().charabase.hp <= 0 {
                    channel_id
                        .send_message(&ctx.http, |f| {
                            f.embed(|e| {
                                e.title(format!(
                                    "{}に倒されてしまった",
                                    result.enemy().charabase.name
                                ))
                            })
                        })
                        .await
                        .context("埋め込みの作成に失敗しました")?;
                    battle.reset_turn();
                    break;
                } else {
                    break;
                }
            }
        }
    }
    Ok(())
}

pub async fn delete(
    ctx: &serenity::client::Context,
    channel_id: ChannelId,
    user: User,
    postgres_connect: sea_orm::DatabaseConnection,
) -> CommandResult {
    let question = channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title("本当に削除してもよろしいでしょうか？")
                    .description("削除したデータは二度と戻ってきません")
            })
            .reactions(YES_NO_REACTIONS.to_vec())
        })
        .await
        .context("埋め込みの作成に失敗しました")?;

    if let Some(reaction) = &question
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(
            config_parse_toml().await.timeout_duration().unwrap_or(10),
        ))
        .author_id(user.id.0)
        .await
    {
        let emoji = &reaction.as_inner_ref().emoji;
        match emoji.as_data().as_str() {
            "⭕" => {
                match UserDataEntity::find_by_id(user.id.to_string())
                    .one(&postgres_connect)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?
                {
                    Some(ud) => {
                        ud.delete(&postgres_connect).await;
                    }
                    None => todo!(),
                }
            }
            "❌" => {
                channel_id
                    .send_message(&ctx.http, |f| f.embed(|e| e.title("削除を取り消します")))
                    .await
                    .context("埋め込みの作成に失敗しました")?;
            }
            _ => {
                error_embed_message(ctx, channel_id, "正しい反応を選んで下さい").await?;
            }
        }
    }

    Ok(())
}

pub async fn setchara(
    ctx: &serenity::client::Context,
    channel_id: ChannelId,
    chara: String,
    user: User,
    postgres_connect: sea_orm::DatabaseConnection,
) -> CommandResult {
    let chara_data = CharaConfig::chara_new(&chara)
        .await
        .context("Invalid arg")?;
    channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title(format!(
                    "キャラクターを{}に変更しました",
                    &chara_data.charabase.name
                ))
                .description(" ")
            })
        })
        .await
        .context("埋め込みの作成に失敗しました")?;
    let userdata = Model::get(&postgres_connect, user.id.0.to_string()).await;

    if let Some(data) = userdata {
        let new_model = Model {
            player: chara_data.charabase.name,
            ..data
        };
        new_model.update_player(&postgres_connect).await;
    } else {
        Model {
            user_id: user.id.to_string(),
            player: chara_data.charabase.name,
            level: 1,
            exp: 1,
            battle_uuid: None,
        }
        .update_player(&postgres_connect)
        .await;
    }
    Ok(())
}
/// 操作の埋め込み
async fn operation_enemy(
    ctx: &serenity::client::Context,
    channel_id: ChannelId,
    reactions: Vec<ReactionType>,
) -> Result<Message, anyhow::Error> {
    channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("リアクションを押して操作してね").description(" "))
                .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

async fn error_embed_message<M: Into<String>>(
    ctx: &serenity::client::Context,
    channel_id: ChannelId,
    context: M,
) -> Result<Message, anyhow::Error> {
    channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("エラーが発生しました").description(context.into()))
        })
        .await
        .context("埋め込みの作成に失敗しました")
}
