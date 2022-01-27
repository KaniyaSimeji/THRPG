mod command;
mod database;
mod setting;
mod battle;

use std::env;

use serenity::{
    framework::StandardFramework, 
    async_trait, 
    client::EventHandler, 
    Client, 
};

use command::play::GENERAL_GROUP;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}    

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
    .configure(|c| c.prefix("th!")).group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN").expect("Not found token");

    let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework).await.expect("Error client");

    if let Err(why) = client.start().await {
        println!("{:?}",why)
    }
}
