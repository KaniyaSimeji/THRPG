use crate::battle::{
    builder::{BattleBuilder, RandomOption},
    model::CharaConfig,
    rpg_core::PlayMode,
};
use crate::database::{
    playdata::Entity as PlaydataEntity,
    postgres_connect,
    save::{delete as userdata_delete, save, update_player, Entity as UserDataEntity, Model},
};
use crate::setting::{
    i18n::i18n_text,
    setup::{config_parse_toml, Languages},
};
use anyhow::Context;
use once_cell::sync::Lazy;
use sea_orm::EntityTrait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
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

#[group]
#[commands(play, delete, setchara)]
pub struct General;

/// play
#[command]
#[description = "ゲームをプレイする"]
pub async fn play(ctx: &serenity::client::Context, msg: &Message, args: Args) -> CommandResult {
    if !msg.author.bot {
        let postgresql_config = config_parse_toml().await.postgresql_config();
        let userdata = match &postgresql_config {
            Some(f) => {
                let db_address = f.db_address.as_ref().unwrap();
                let db_conn = postgres_connect::connect(db_address)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                match UserDataEntity::find_by_id(msg.author.id.as_u64().to_string())
                    .one(&db_conn)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?
                {
                    Some(ud) => Some(ud),
                    None => {
                        let model = Model {
                            exp: 1,
                            level: 1,
                            player: "Reimu".to_string(),
                            user_id: msg.author.id.as_u64().to_string(),
                            battle_uuid: None,
                        };
                        save(&db_conn, model.clone()).await;
                        Some(model)
                    }
                }
            }
            None => None,
        };

        let playdata = match &postgresql_config {
            Some(f) => {
                let db_address = f.db_address.as_ref().unwrap();
                let db_conn = postgres_connect::connect(db_address)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                match &userdata {
                    Some(r) => match r.battle_uuid {
                        Some(r) => PlaydataEntity::find_by_id(r)
                            .one(&db_conn)
                            .await
                            .map_err(|e| anyhow::anyhow!(e))?,
                        None => None,
                    },
                    None => None,
                }
            }
            None => None,
        };

        let mut battle = match playdata {
            Some(d) => {
                let builder: BattleBuilder = d.into();
                builder.build()
            }
            None => {
                let arg_playmode = match &args.current() {
                    Some(s) => match PlayMode::try_from_value(s) {
                        Ok(g) => Some(g),
                        Err(e) => {
                            error_embed_message(&ctx, &msg, format!("{} is not found", e)).await?;
                            None
                        }
                    },
                    None => None,
                };
                let mut init = BattleBuilder::new(
                    match arg_playmode {
                        Some(r) => r,
                        None => PlayMode::Simple,
                    },
                    match &userdata {
                        Some(d) => Some(d.into()),
                        None => None,
                    },
                    None,
                    None,
                );

                init.enemy_random(RandomOption::default()).await;

                init.build()
            }
        };

        msg.channel_id
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
                    operation_enemy(&ctx, &msg, BATTLE_REACTIONS.to_vec()).await?;
                if let Some(reaction) = &operation_embed
                    .await_reaction(&ctx)
                    .timeout(Duration::from_secs(
                        config_parse_toml().await.timeout_duration().unwrap_or(10),
                    ))
                    .author_id(msg.author.id)
                    .await
                {
                    let emoji = &reaction.as_inner_ref().emoji;
                    match emoji.as_data().as_str() {
                        BATTLE_PLAY => {
                            let result = battle.result_battle().await;
                            if result.enemy().charabase.hp > 0 {
                                msg.channel_id
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
                                msg.channel_id
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
                            msg.channel_id
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
                        BATTLE_SAVE => match config_parse_toml().await.postgresql_config() {
                            Some(url) => {
                                let url_string = url.db_address.unwrap();
                                let dbconn = postgres_connect::connect(url_string)
                                    .await
                                    .expect("Invelid URL");

                                save(
                                    &dbconn,
                                    Model {
                                        user_id: msg.author.id.0.to_string(),
                                        exp: match userdata.as_ref() {
                                            Some(e) => e.exp,
                                            None => 1,
                                        },
                                        level: match userdata.as_ref() {
                                            Some(l) => l.level,
                                            None => 1,
                                        },
                                        player: match userdata.as_ref() {
                                            Some(p) => p.player.clone(),
                                            None => "Reimu".to_string(),
                                        },
                                        battle_uuid: Some(battle.uuid()),
                                    },
                                )
                                .await;

                                let question = msg
                                    .channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            e.title("thrpgを続けますか？")
                                                .description("セーブされているので続きをプレイすることも可能です")
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
                                    .author_id(msg.author.id)
                                    .await
                                {
                                    let emoji = &reaction.as_inner_ref().emoji;
                                    match emoji.as_data().as_str() {
                                        "❌" => {
                                            break;
                                        }
                                        "⭕" => {
                                            msg.channel_id
                                                .send_message(&ctx.http, |f| {
                                                    f.embed(|e| e.title("thrpgを続けます"))
                                                })
                                                .await
                                                .context("埋め込みの作成に失敗しました")?;
                                        }
                                        _ => {
                                            error_embed_message(
                                                ctx,
                                                msg,
                                                "正しい反応を選んで下さい",
                                            )
                                            .await?;
                                        }
                                    }
                                }
                            }
                            None => {
                                error_embed_message(ctx, msg, "データベースに接続できません")
                                    .await
                                    .unwrap();
                                break;
                            }
                        },
                        _ => break,
                    }
                }
            } else if battle.elapesd_turns() == 0 && battle.turn() == battle.enemy() {
                let operation_embed =
                    operation_enemy(&ctx, &msg, BATTLE_REACTIONS.to_vec()).await?;
                battle.add_turn();
                if let Some(reaction) = &operation_embed
                    .await_reaction(&ctx)
                    .timeout(Duration::from_secs(
                        config_parse_toml().await.timeout_duration().unwrap_or(10),
                    ))
                    .author_id(msg.author.id)
                    .await
                {
                    let emoji = &reaction.as_inner_ref().emoji;
                    match emoji.as_data().as_str() {
                        BATTLE_PLAY => {
                            let result = battle.result_battle().await;
                            if result.enemy().charabase.hp > 0 {
                                msg.channel_id
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
                                msg.channel_id
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
                            msg.channel_id
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
                        BATTLE_SAVE => match config_parse_toml().await.postgresql_config() {
                            Some(url) => {
                                let url_string = url.db_address.unwrap();
                                let dbconn = postgres_connect::connect(url_string)
                                    .await
                                    .expect("Invelid URL");

                                save(
                                    &dbconn,
                                    crate::database::save::Model {
                                        user_id: msg.author.id.0.to_string(),
                                        exp: match userdata.as_ref() {
                                            Some(e) => e.exp,
                                            None => 1,
                                        },
                                        level: match userdata.as_ref() {
                                            Some(l) => l.level,
                                            None => 1,
                                        },
                                        player: match userdata.as_ref() {
                                            Some(p) => p.player.clone(),
                                            None => "Reimu".to_string(),
                                        },
                                        battle_uuid: Some(battle.uuid()),
                                    },
                                )
                                .await;

                                let question = msg
                                    .channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            e.title("thrpgを続けますか？")
                                                .description("セーブされているので続きをプレイすることも可能です")
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
                                    .author_id(msg.author.id)
                                    .await
                                {
                                    let emoji = &reaction.as_inner_ref().emoji;
                                    match emoji.as_data().as_str() {
                                        "❌" => {
                                            break;
                                        }
                                        "⭕" => {
                                            msg.channel_id
                                                .send_message(&ctx.http, |f| {
                                                    f.embed(|e| e.title("thrpgを続けます"))
                                                })
                                                .await
                                                .context("埋め込みの作成に失敗しました")?;
                                        }
                                        _ => {
                                            error_embed_message(
                                                ctx,
                                                msg,
                                                "正しい反応を選んで下さい",
                                            )
                                            .await?;
                                        }
                                    }
                                }
                            }
                            None => {
                                error_embed_message(ctx, msg, "データベースに接続できません")
                                    .await
                                    .unwrap();
                                break;
                            }
                        },
                        _ => break,
                    }
                }
            } else {
                let result = battle.result_battle().await;
                if result.player().charabase.hp > 0 {
                    msg.channel_id
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
                    msg.channel_id
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

#[command]
#[description = "セーブデータを削除する"]
pub async fn delete(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    let question = msg
        .channel_id
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
        .author_id(msg.author.id)
        .await
    {
        let emoji = &reaction.as_inner_ref().emoji;
        match emoji.as_data().as_str() {
            "⭕" => match config_parse_toml().await.postgresql_config() {
                Some(url) => {
                    let url_string = url.db_address.unwrap();
                    let dbconn = postgres_connect::connect(url_string)
                        .await
                        .expect("Invelid URL");
                    userdata_delete(&dbconn, *msg.author.id.as_u64()).await;
                }
                None => {
                    error_embed_message(ctx, msg, "データベースに接続できません").await?;
                }
            },
            "❌" => {
                msg.channel_id
                    .send_message(&ctx.http, |f| f.embed(|e| e.title("削除を取り消します")))
                    .await
                    .context("埋め込みの作成に失敗しました")?;
            }
            _ => {
                error_embed_message(ctx, msg, "正しい反応を選んで下さい").await?;
            }
        }
    }

    Ok(())
}

#[command]
#[description = "キャラクターを選択します"]
pub async fn setchara(
    ctx: &serenity::client::Context,
    msg: &Message,
    mut arg: Args,
) -> CommandResult {
    let arg_str = arg.trimmed().current();
    if let Some(arg) = arg_str {
        let chara_data = CharaConfig::chara_new(&arg.to_string())
            .await
            .context("Invalid arg")?;
        msg.channel_id
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

        match config_parse_toml().await.postgresql_config() {
            Some(url) => {
                let url_string = url.db_address.unwrap();
                let dbconn = postgres_connect::connect(url_string)
                    .await
                    .expect("Invelid URL");
                update_player(&dbconn, msg.author.id.0, chara_data.charabase.name).await;
            }
            None => {
                error_embed_message(ctx, msg, "データベースに接続できません").await?;
            }
        }
    } else {
        error_embed_message(&ctx, &msg, "何のキャラクターも選択されていません").await?;
    }

    Ok(())
}
/// 操作の埋め込み
async fn operation_enemy(
    ctx: &serenity::client::Context,
    msg: &Message,
    reactions: Vec<ReactionType>,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("リアクションを押して操作してね").description(" "))
                .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

async fn error_embed_message<M: Into<String>>(
    ctx: &serenity::client::Context,
    msg: &Message,
    context: M,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("エラーが発生しました").description(context.into()))
        })
        .await
        .context("埋め込みの作成に失敗しました")
}
