use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "ping値を送る"]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    message
        .channel_id
        .say(&context.http, format!("{} ping!", message.author.mention()))
        .await?;

    Ok(())
}
