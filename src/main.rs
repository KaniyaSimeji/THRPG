use std::{collections::HashSet, env};

use ::serenity::{
    async_trait,
    framework::standard::{
        macros::{group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use serenity::framework::{standard::help_commands, StandardFramework};

mod cmds;
use cmds::ping::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is start", ready.user.name);
    }
}

#[group]
#[description("管理系")]
#[summary("一般")]
#[commands(ping)]
struct General;

#[help]
#[individual_command_tip = "help command"]
#[strikethrough_commands_tip_in_guild = ""]
async fn my_help(
    context: &Context,
    message: &Message,
    args: Args,
    help_option: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, message, args, help_option, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token nai...");
    let framework = StandardFramework::new()
        .configure(|a| a.prefix("m."))
        .help(&MY_HELP)
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("client dame");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
