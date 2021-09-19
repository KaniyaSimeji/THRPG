use serenity::{
    client::{Client, Context},
    framework::standard::{macros, Args, Check, CommandResult, StandardFramework},
    http::Http,
    model::{channel::Message, gateway::Ready, misc::Mentionable, prelude::ChannelId},
    Result as SerenityResult,
};

use songbird::{
    input::{self as songbirdself, restartable::Restart},
    Event, EventContext, EventHandler as VoiceEventHamdle, SerenityInit, TrackEvent,
};

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

async fn join(context: &Context, message: &Message) -> CommandResult {
    let GuildGet = message.guild(&context.cache).await.unwrap();
    let GuildIdGet = GuildGet.id;

    let ChannelIdGet = GuildGet
        .voice_states
        .get(&message.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let ConnectTo = match ChannelIdGet {
        Some(channel) => channel,
        None => {
            check_msg(
                message
                    .reply(context, "I can't get into the voice channel...")
                    .await,
            );
            return Ok(());
        }
    };
}
