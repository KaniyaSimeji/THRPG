mod battle;
mod command;
mod database;
mod log;
mod setting;

use std::collections::HashSet;
use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{
        standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    model::prelude::{Message, UserId},
    Client,
};

use command::author::RELATION_GROUP;
use command::play::GENERAL_GROUP;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("th!"))
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&RELATION_GROUP);

    let discord_token = setting::setup::config_parse_toml().await.token();

    let mut client = Client::builder(&discord_token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error client");

    if let Err(why) = client.start().await {
        println!("{:?}", why)
    }
}

#[serenity::framework::standard::macros::help]
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
