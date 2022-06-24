use battle_machine::rpg_core::PlayMode;
use bot_command::{
    info::info,
    play::{delete, play},
};
use extension::{
    extension_manage::{ExtensionAuthority, ExtensionManager},
    store::ExtensionStore,
};
use once_cell::sync::Lazy;
use setting_config::Config;
use std::collections::HashSet;
use thrpg_database::postgres_connect::connect;
use wasmer::Exports;

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

static EXPORT_FUNCTIONS: Lazy<Exports> = Lazy::new(|| Exports::new());

struct Handler {
    config: Config,
    extensions: ExtensionManager,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing(
            self.config.prefix().unwrap_or(&"th!".to_string()),
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
                                .kind(ApplicationCommandOptionType::String)
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
                                .kind(ApplicationCommandOptionType::SubCommand)
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
async fn main() -> anyhow::Result<()> {
    let extension_store = ExtensionStore::extension_files().await?;
    let config = setting_config::config_parse_toml().await;
    let extension_manage = ExtensionManager::to_manager(
        extension_store,
        &*EXPORT_FUNCTIONS,
        ExtensionAuthority::authority(config.authority_strict(), config.authority_flags()),
    );

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(config.clone().prefix().unwrap_or(&"th!".to_string())))
        .help(&HELP);

    let handler = Handler {
        config: config.clone(),
        extensions: extension_manage,
    };

    let mut client = Client::builder(config.token(), GatewayIntents::all())
        .event_handler(handler)
        .framework(framework)
        .await
        .map_err(|e| anyhow::anyhow!("client can't start: {:?}", e))?;

    client.start().await.map_err(|e| anyhow::anyhow!(e))
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
