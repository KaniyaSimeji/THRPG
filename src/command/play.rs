use crate::battle::{
    builder::{BattleBuilder, RandomOption},
    model::CharaConfig,
    rpg_core::PlayMode,
};
use crate::database::{
    playdata::Entity as PlaydataEntity,
    postgres_connect,
    save::{delete as userdata_delete, save, update_player, Entity},
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

pub fn battle_reactions() -> Vec<ReactionType> {
    let mut vec = Vec::new();
    vec.push(ReactionType::Unicode(BATTLE_PLAY.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_GUARD.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_ITEM.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_SAVE.to_string()));
    vec
}

static BATTLEREACTIONS: Lazy<Vec<ReactionType>> = Lazy::new(|| {
    let mut vec = Vec::new();
    vec.push(ReactionType::Unicode(BATTLE_PLAY.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_GUARD.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_ITEM.to_string()));
    vec.push(ReactionType::Unicode(BATTLE_SAVE.to_string()));
    vec
});

const BATTLE_PLAY: &str = "⚔";
const BATTLE_ITEM: &str = "💊";
const BATTLE_SAVE: &str = "✒️";
const BATTLE_GUARD: &str = "\u{1F6E1}";

#[group]
#[commands(play, delete)]
pub struct General;

/// play
#[command]
#[description = "ゲームをプレイする"]
pub async fn play(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    if !msg.author.bot {
        let postgresql_config = config_parse_toml().await.postgresql_config();
        let userdata = match &postgresql_config {
            Some(f) => {
                let db_address = f.db_address.as_ref().unwrap();
                let dbconn = postgres_connect::connect(db_address)
                    .await
                    .expect("Invelid URL");
                Entity::find_by_id(msg.author.id.0.to_string())
                    .one(&dbconn)
                    .await?
            }
            None => None,
        };

        let battledata = match postgresql_config {
            Some(f) => {
                let db_address = f.db_address.unwrap();
                let dbconn = postgres_connect::connect(db_address)
                    .await
                    .expect("Invelid URL");
                match &userdata {
                    Some(m) => match m.battle_uuid {
                        Some(u) => PlaydataEntity::find_by_id(u).one(&dbconn).await?,
                        None => None,
                    },
                    None => None,
                }
            }
            None => None,
        };

        let mut battle = match battledata {
            Some(d) => {
                let builder: BattleBuilder = d.into();
                let battle = builder.build();
                msg.channel_id
                    .send_message(&ctx.http, |f| {
                        f.embed(|e| {
                            e.title(format!(
                                "{}とのバトルを再開します",
                                battle.enemy().charabase.name
                            ))
                            .description(format!("{}ターン目です", battle.elapesd_turns()))
                        })
                    })
                    .await?;
                battle
            }
            None => {
                let mut init = BattleBuilder::new(
                    PlayMode::Simple,
                    match &userdata {
                        Some(d) => Some(d.clone().into()),
                        None => None,
                    },
                    None,
                    None,
                );
                let battle_builder = init
                    .player(CharaConfig::chara_new("Reimu").await?)
                    .enemy_random(RandomOption::default())
                    .await
                    .clone();

                let battle = battle_builder.build();
                // 敵の出現
                msg.channel_id
                    .send_message(&ctx.http, |f| {
                        f.embed(|e| {
                            e.title(format!(
                                "{enemy}{appear_enemy}",
                                enemy = &battle.enemy().charabase.name,
                                appear_enemy =
                                    i18n_text(Languages::Japanese).game_message.appear_enemy
                            ))
                            .description("ank".to_string())
                        })
                    })
                    .await?;
                battle
            }
        };

        let mut msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?;

        if !battle.is_running() {
            loop {
                // もし絵文字が付いたら行う処理
                if let Some(reaction) = &msg_embed
                    .await_reaction(&ctx)
                    .timeout(Duration::from_secs(
                        config_parse_toml().await.timeout_duration().unwrap_or(10),
                    ))
                    .author_id(msg.author.id)
                    .await
                {
                    let emoji = &reaction.as_inner_ref().emoji;
                    let _ = match emoji.as_data().as_str() {
                        BATTLE_PLAY => {
                            let battle_clone = battle.clone();
                            let result = battle.result_battle().await;

                            if battle_clone.enemy().charabase.hp != result.enemy().charabase.hp
                                && result.enemy().charabase.hp > 0
                            {
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
                                battle.add_turn();

                                msg_embed = operation_enemy(&ctx, &msg, battle_reactions())
                                    .await
                                    .unwrap()
                            } else if battle_clone.player().charabase.hp
                                != result.player().charabase.hp
                                && result.player().charabase.hp > 0
                            {
                                msg.channel_id
                                    .send_message(&ctx.http, |f| {
                                        f.embed(|e| {
                                            let player = result.player();
                                            e.title(format!(
                                                "味方ののこりhp{}",
                                                player.charabase.hp
                                            ))
                                            .description(&player.charabase.name)
                                        })
                                    })
                                    .await
                                    .context("埋め込みの作成に失敗しました")?;
                                battle.add_turn();

                                msg_embed = operation_enemy(&ctx, &msg, battle_reactions())
                                    .await
                                    .unwrap()
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
                                break;
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
                                break;
                            } else {
                                break;
                            }
                        }
                        BATTLE_GUARD => {
                            guard_attack(&ctx, &msg).await.unwrap();
                            msg_embed = operation_enemy(&ctx, &msg, battle_reactions())
                                .await
                                .unwrap()
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
                            }
                            None => {
                                error_embed_message(ctx, msg, "データベースに接続できません")
                                    .await
                                    .unwrap();
                                break;
                            }
                        },
                        _ => {
                            break;
                        }
                    };
                }
            }
        }
    }

    Ok(())
}

#[command]
#[description = "セーブデータを削除する"]
pub async fn delete(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    let reactions = [
        ReactionType::Unicode("⭕".to_string()),
        ReactionType::Unicode("❌".to_string()),
    ];

    let question = msg
        .channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title("本当に削除してもよろしいでしょうか？")
                    .description("削除したデータは二度と戻ってきません")
            })
            .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")?;

    if let Some(reaction) = &question
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(10))
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
pub async fn set_chara(
    ctx: &serenity::client::Context,
    msg: &Message,
    mut arg: Args,
) -> CommandResult {
    let arg_str = arg.trimmed().current().context("Not found arg")?;
    let chara_data = todo!(); /* CharaBase::chara_new(&arg_str.to_string())
                              .await
                              .context("Invalid arg")?; */

    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title(format!("キャラクターを{}に変更しました", arg_str))
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
            update_player(&dbconn, msg.author.id.0, todo!()).await;
        }
        None => {
            error_embed_message(ctx, msg, "データベースに接続できません").await?;
        }
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
            f.embed(|e| {
                e.title("リアクションを押して操作してね")
                    .description("meme")
            })
            .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

async fn guard_attack(
    ctx: &serenity::client::Context,
    msg: &Message,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("{}ダメージをくらった").description("{}%防御した"))
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
