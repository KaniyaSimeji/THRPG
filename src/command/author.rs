use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::Message;

#[group]
#[commands(info)]
pub struct Relation;

#[command]
pub async fn info(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    if !msg.author.bot {
        msg.channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| e.title("aaa".to_string()).description("ank".to_string()))
            })
            .await?;
    }
    Ok(())
}
