use crate::setting::setup::BOTInfo;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{ChannelId, User};

pub async fn info(ctx: Context, channel_id: ChannelId, user: User) -> CommandResult {
    if !user.bot {
        tokio::spawn(
        channel_id
            .send_message(ctx.http, |f| {
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
        ).await??;
    }
    Ok(())
}
