use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{ChannelId, User};

pub async fn info(ctx: Context, channel_id: ChannelId, user: User) -> CommandResult {
    if !user.bot {
        tokio::spawn(
        channel_id
            .send_message(ctx.http, |f| {
                f.embed(|e| {
                    e.title(format!(
                        "thrpgは以下のメンテナーと多数のコントリビューターによって支えられています",
                    ))
                    .description(format!(
                        "{}\n [多数のコントリビューター](https://github.com/thrpg/thrpg)\n そして拡張機能やプレイしてくれている皆さんに感謝！",
                        "KaniyaSimeji"
                    ))
                })
            })
        ).await??;
    }
    Ok(())
}
