use crate::setting::setup::BOTInfo;
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::Message;

#[group]
#[commands(info)]
pub struct Relation;

#[command]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    if !msg.author.bot {
        msg.channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| {
                    let botinfo = BOTInfo::info();
                    e.title(format!(
                        "{}は以下のメンテナーと多数のコントリビューターによって支えられています",
                        botinfo.name
                    ))
                    .description(format!(
                        "{}\n [多数のコントリビューター](https://github.com/thrpg/thrpg)\n そして拡張機能やプレイしてくれている皆さんに感謝！",
                        botinfo.author
                    ))
                })
            })
            .await?;
    }
    Ok(())
}
