use std::collections::HashSet;
use thrpg::battle::{builder::BattleBuilder, rpg_core::PlayMode};
use thrpg::command::{
    infos::info,
    play::{delete, play},
};
use thrpg::database::postgres_connect::connect;
use thrpg::extension::wasm::wasm_init;
use thrpg::setting::setup;
use thrpg::setting::setup::Config;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::{
        standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
        StandardFramework,
    },
    model::{
        gateway::{GatewayIntents, Ready},
        interactions::application_command::ApplicationCommandOptionType,
        prelude::{application_command::ApplicationCommand, Interaction, Message, UserId},
    },
    Client,
};

struct Handler {
    config: Config,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing(
            setup::config_parse_toml()
                .await
                .prefix()
                .unwrap_or(&"th!".to_string()),
        ))
        .await;

        let _ = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("play")
                        .description("play the thrpg!!")
                        .create_option(|option| {
                            option
                                .name("chara")
                                .description("Select the character you want to operate")
                                .kind(ApplicationCommandOptionType::String)
                        })
                        .create_option(|option| {
                            option
                                .name("gamemode")
                                .description("Select game mode")
                                .kind(ApplicationCommandOptionType::String)
                                .add_string_choice("simple", PlayMode::Simple)
                                .add_string_choice("raid", PlayMode::Raid)
                        })
                        .create_option(|option| {
                            option
                                .name("story")
                                .description("Select story!!")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("delete")
                        .description("delete player's account")
                })
                .create_application_command(|command| {
                    command
                        .name("chara")
                        .description("player chara setting")
                        .create_option(|option| {
                            option
                                .name("set_chara_name")
                                .description("The character you want to operate")
                                .required(true)
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("status")
                        .description("your status!!")
                        .create_option(|option| {
                            option
                                .name("main")
                                .description("simple your status")
                                .kind(ApplicationCommandOptionType::SubCommand)
                        })
                        .create_option(|option| {
                            option
                                .name("score")
                                .description("your game score")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("info")
                        .description("thrpg information")
                        .create_option(|option| {
                            option
                                .name("story")
                                .description("Story information")
                                .kind(ApplicationCommandOptionType::String)
                        })
                        .create_option(|option| {
                            option
                                .name("extension")
                                .description("Extension information")
                                .kind(ApplicationCommandOptionType::String)
                        })
                        .create_option(|option| {
                            option
                                .name("content")
                                .description("Content information")
                                .kind(ApplicationCommandOptionType::String)
                        })
                        .create_option(|option| {
                            option
                                .name("thrpg")
                                .description("thrpg information")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "play" => play(
                    ctx,
                    command.channel_id,
                    command.user,
                    connect(self.config.postgresql_config().db_address)
                        .await
                        .unwrap(),
                )
                .await
                .unwrap(),
                "info" => {
                    info(ctx, command.channel_id, command.user).await.unwrap();
                }
                "delete" => delete(
                    &ctx,
                    command.channel_id,
                    command.user,
                    connect(self.config.postgresql_config().db_address)
                        .await
                        .unwrap(),
                )
                .await
                .unwrap(),
                _ => todo!(),
            };
        }
    }
}

#[tokio::main]
async fn main() {
    let config = setup::config_parse_toml().await;
    if let Err(why) = wasm_init().await {
        println!("{:?}", why)
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.clone().prefix().unwrap_or(&"th!".to_string())))
        .help(&HELP);

    let handler = Handler {
        config: config.clone(),
    };

    let mut client = Client::builder(config.token(), GatewayIntents::all())
        .event_handler(handler)
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
