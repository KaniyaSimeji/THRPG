use std::time::Duration;

use anyhow::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;

const BATTLE_PLAY: &str = "⚔";
const BATTLE_SAVE: &str = "✒️";
const BATTLE_ITEM: &str = "⚗️";
const BATTLE_GUARD: &str = "\u{1F6E1}";

#[group]
#[commands(ping, play)]
pub struct General;

#[command]
pub async fn ping(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "hello").await?;

    Ok(())
}
#[command]
pub async fn play(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    let _ = msg
        .channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title(format!("{}が現れた！", "ank"))
                    .description(format!("{}", "ank"))
            })
        })
        .await?;

    let reactions = vec![
        ReactionType::Unicode(BATTLE_PLAY.to_string()),
        ReactionType::Unicode(BATTLE_GUARD.to_string()),
        ReactionType::Unicode(BATTLE_ITEM.to_string()),
        ReactionType::Unicode(BATTLE_SAVE.to_string()),
    ];
    let mut msg_embed = operation_enemy(ctx, msg, reactions).await?;
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
                    let _reactions = vec![
                        ReactionType::Unicode(BATTLE_PLAY.to_string()),
                        ReactionType::Unicode(BATTLE_GUARD.to_string()),
                        ReactionType::Unicode(BATTLE_ITEM.to_string()),
                        ReactionType::Unicode(BATTLE_SAVE.to_string()),
                    ];

                    msg_embed = operation_enemy(ctx, msg, _reactions).await?
                }
                _ => break,
            };
        }
    }

    Ok(())
}

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
