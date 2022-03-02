use crate::battle::charabase::random_enemy;
use crate::database::{postgres_connect, save::save};
use crate::setting::{
    i18n::i18n_text,
    setup::{config_parse_toml, Languages},
};
use std::time::Duration;

use anyhow::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;

/// バトルコマンドの時に使うコマンドの配列の取得
pub fn battle_reactions() -> [ReactionType; 4] {
    [
        ReactionType::Unicode(BATTLE_PLAY.to_string()),
        ReactionType::Unicode(BATTLE_GUARD.to_string()),
        ReactionType::Unicode(BATTLE_ITEM.to_string()),
        ReactionType::Unicode(BATTLE_SAVE.to_string()),
    ]
}
const BATTLE_PLAY: &str = "⚔";
const BATTLE_SAVE: &str = "✒️";
const BATTLE_ITEM: &str = "⚗️";
const BATTLE_GUARD: &str = "\u{1F6E1}";

#[group]
#[commands(ping, play)]
pub struct General;

#[command]
pub async fn ping(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    if msg.author.bot != true {
        msg.reply(ctx, "hello").await?;
    }
    Ok(())
}

/// play
#[command]
#[description = "ゲームをプレイする"]
pub async fn play(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    let chara_name = random_enemy("chara").await.unwrap().name;
    let appear_enemy = i18n_text(Languages::Japanese)
        .await
        .game_message
        .appear_enemy;

    if msg.author.bot != true {
        // 敵の出現
        let _ = msg
            .channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| {
                    {
                        e.title(format!("{chara_name}{appear_enemy}"))
                            .description("ank".to_string())
                    }
                })
            })
            .await?;

        let mut msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?;

        // もし絵文字が付いたら行う処理
        loop {
            if let Some(reaction) = &msg_embed
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(10))
                .author_id(msg.author.id)
                .await
            {
                let emoji = &reaction.as_inner_ref().emoji;
                let _ = match emoji.as_data().as_str() {
                    BATTLE_PLAY => {
                        result_battle(ctx, msg).await?;
                        msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?
                    }
                    BATTLE_GUARD => {
                        guard_attack(ctx, msg).await?;
                        msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?
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
                                    user_id: *msg.author.id.as_u64() as i64,
                                    exp: 5,
                                    level: 3,
                                    player: "Sakuya".to_string(),
                                },
                            )
                            .await;
                        }
                        None => {
                            error_embed_message(ctx, msg, "データベースに接続できません").await?;
                            break;
                        }
                    },
                    _ => break,
                };
            }
        }
    }

    Ok(())
}

/// 操作の埋め込み
async fn operation_enemy(
    ctx: &serenity::client::Context,
    msg: &Message,
    reactions: [ReactionType; 4],
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

/// 結果の埋め込み
async fn result_battle(
    ctx: &serenity::client::Context,
    msg: &Message,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("結果は{}でした").description("特記事項はなし！"))
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
