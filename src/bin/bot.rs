use std::collections::HashSet;
use thrpg::command::{author::RELATION_GROUP, play::GENERAL_GROUP};
use thrpg::setting::setup;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{
        standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: serenity::model::gateway::Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing(
            setup::config_parse_toml()
                .await
                .prefix()
                .unwrap_or("th!".to_string()),
        ))
        .await;
    }
}

#[tokio::main]
async fn main() {
    let prefix = setup::config_parse_toml()
        .await
        .prefix()
        .unwrap_or("th!".to_string());
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(prefix))
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&RELATION_GROUP);

    let discord_token = setup::config_parse_toml().await.token();

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error client");

    if let Err(why) = client.start().await {
        println!("{:?}", why)
    }
}

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
