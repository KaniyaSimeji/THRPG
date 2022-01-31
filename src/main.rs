mod battle;
mod command;
mod database;
mod log;
mod setting;

use std::env;

use serenity::{async_trait, client::EventHandler, framework::StandardFramework, Client};

use command::play::GENERAL_GROUP;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("th!"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Not found token");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error client");

    if let Err(why) = client.start().await {
        println!("{:?}", why)
    }
}
