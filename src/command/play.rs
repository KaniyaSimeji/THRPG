use crate::database::{
    playdata::Entity as PlaydataEntity,
    postgres_connect,
    save::{delete as userdata_delete, save, update_player, Entity},
};
use crate::setting::{
    i18n::i18n_text,
    setup::{config_parse_toml, Languages},
};
use anyhow::Context;
use chrono::prelude::Local;
use sea_orm::EntityTrait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use std::time::Duration;

/// バトルコマンドの時に使うコマンドの配列の取得
pub fn battle_reactions() -> [ReactionType; 4] {
    [
        ReactionType::Unicode(BATTLE_PLAY.to_string()),
        ReactionType::Unicode(BATTLE_GUARD.to_string()),
        ReactionType::Unicode(BATTLE_ITEM.to_string()),
        ReactionType::Unicode(BATTLE_SAVE.to_string()),
    ]
}
const BATTLE_PLAY: &str = "⚔";
const BATTLE_ITEM: &str = "💊";
const BATTLE_SAVE: &str = "✒️";
const BATTLE_GUARD: &str = "\u{1F6E1}";

#[group]
#[commands(play, delete, set_chara)]
pub struct General;

/// play
#[command]
#[description = "ゲームをプレイする"]
pub async fn play(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    if !msg.author.bot {
        //let mut chara_random = random_enemy("chara").await?;

        let userdata = match config_parse_toml().await.postgresql_config() {
            Some(f) => {
                let db_address = f.db_address.unwrap();
                let dbconn = postgres_connect::connect(db_address)
                    .await
                    .expect("Invelid URL");
                Entity::find_by_id(msg.author.id.0.to_string())
                    .one(&dbconn)
                    .await?
            }
            None => None,
        };

        let mut playerdata = match config_parse_toml().await.postgresql_config() {
            Some(f) => {
                let db_address = f.db_address.unwrap();
                let dbconn = postgres_connect::connect(db_address)
                    .await
                    .expect("Invelid URL");
                match &userdata {
                    Some(m) => match m.battle_uuid {
                        Some(u) => PlaydataEntity::find_by_id(u).one(&dbconn).await?,
                        None => None,
                    },
                    None => None,
                }
            }
            None => None,
        };

        // 敵の出現
        match &playerdata {
            Some(d) => {
                msg.channel_id
                    .send_message(&ctx.http, |f| {
                        f.embed(|e| {
                            e.title(format!("{}とのバトルを再開します", d.enemy_name))
                                .description(format!("{}ターン目です", d.elapesd_turns))
                        })
                    })
                    .await?;
            }
            None => {
                msg.channel_id
                    .send_message(&ctx.http, |f| {
                        f.embed(|e| {
                            e.title(format!(
                                "{appear_enemy}",
                                appear_enemy =
                                    i18n_text(Languages::Japanese).game_message.appear_enemy
                            ))
                            .description("ank".to_string())
                        })
                    })
                    .await?;
                let player_name = match userdata.clone() {
                    Some(f) => f.player,
                    None => "Reimu".to_string(),
                };
                let now_datatime = Local::now().naive_local();
                playerdata = Some(crate::database::playdata::Model {
                    battle_id: uuid::Uuid::new_v4(),
                    player_name: player_name.to_string(),
                    enemy_name: todo!(),
                    elapesd_turns: 0,
                    start_time: now_datatime,
                });
            }
        }

        let mut msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?;

        // もし絵文字が付いたら行う処理
        loop {
            if let Some(reaction) = &msg_embed
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(
                    config_parse_toml().await.timeout_duration().unwrap_or(10),
                ))
                .author_id(msg.author.id)
                .await
            {
                let emoji = &reaction.as_inner_ref().emoji;
                let mut player_data = todo!();
                /*
                                    CharaBase::chara_new(&playerdata.as_ref().unwrap().player_name)
                                        .await
                                        .unwrap();
                */
                let _ = match emoji.as_data().as_str() {
                    BATTLE_PLAY => {
                        let battle_state = todo!();
                        // result_battle(ctx, msg, &mut player_data, todo!()).await?;
                        match battle_state {
                            BattleState::BattleContinue => {
                                msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?
                            }
                            BattleState::PlayerDown => break,
                            BattleState::EnemyDown => break,
                        }
                    }
                    BATTLE_GUARD => {
                        guard_attack(ctx, msg).await?;
                        msg_embed = operation_enemy(ctx, msg, battle_reactions()).await?
                    }
                    BATTLE_SAVE => match config_parse_toml().await.postgresql_config() {
                        Some(url) => {
                            let url_string = url.db_address.unwrap();
                            let dbconn = postgres_connect::connect(url_string)
                                .await
                                .expect("Invelid URL");

                            save(
                                &dbconn,
                                crate::database::save::Model {
                                    user_id: msg.author.id.0.to_string(),
                                    exp: match userdata.as_ref() {
                                        Some(e) => e.exp,
                                        None => 1,
                                    },
                                    level: match userdata.as_ref() {
                                        Some(l) => l.level,
                                        None => 1,
                                    },
                                    player: match userdata.as_ref() {
                                        Some(p) => p.player.clone(),
                                        None => "Reimu".to_string(),
                                    },
                                    battle_uuid: match playerdata.as_ref() {
                                        Some(u) => Some(u.battle_id),
                                        None => None,
                                    },
                                },
                            )
                            .await;
                        }
                        None => {
                            error_embed_message(ctx, msg, "データベースに接続できません").await?;
                            break;
                        }
                    },
                    _ => break,
                };
            }
        }
    }

    Ok(())
}

#[command]
#[description = "セーブデータを削除する"]
pub async fn delete(ctx: &serenity::client::Context, msg: &Message) -> CommandResult {
    let reactions = [
        ReactionType::Unicode("⭕".to_string()),
        ReactionType::Unicode("❌".to_string()),
    ];

    let question = msg
        .channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title("本当に削除してもよろしいでしょうか？")
                    .description("削除したデータは二度と戻ってきません")
            })
            .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")?;

    if let Some(reaction) = &question
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(10))
        .author_id(msg.author.id)
        .await
    {
        let emoji = &reaction.as_inner_ref().emoji;
        match emoji.as_data().as_str() {
            "⭕" => match config_parse_toml().await.postgresql_config() {
                Some(url) => {
                    let url_string = url.db_address.unwrap();
                    let dbconn = postgres_connect::connect(url_string)
                        .await
                        .expect("Invelid URL");
                    userdata_delete(&dbconn, *msg.author.id.as_u64()).await;
                }
                None => {
                    error_embed_message(ctx, msg, "データベースに接続できません").await?;
                }
            },
            "❌" => {
                msg.channel_id
                    .send_message(&ctx.http, |f| f.embed(|e| e.title("削除を取り消します")))
                    .await
                    .context("埋め込みの作成に失敗しました")?;
            }
            _ => {
                error_embed_message(ctx, msg, "正しい反応を選んで下さい").await?;
            }
        }
    }

    Ok(())
}

#[command]
#[description = "キャラクターを選択します"]
pub async fn set_chara(
    ctx: &serenity::client::Context,
    msg: &Message,
    mut arg: Args,
) -> CommandResult {
    let arg_str = arg.trimmed().current().context("Not found arg")?;
    let chara_data = todo!(); /* CharaBase::chara_new(&arg_str.to_string())
                              .await
                              .context("Invalid arg")?; */

    let _ = msg
        .channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title(format!("キャラクターを{}に変更しました", todo!()))
                    .description(" ")
            })
        })
        .await
        .context("埋め込みの作成に失敗しました")?;

    match config_parse_toml().await.postgresql_config() {
        Some(url) => {
            let url_string = url.db_address.unwrap();
            let dbconn = postgres_connect::connect(url_string)
                .await
                .expect("Invelid URL");
            update_player(&dbconn, msg.author.id.0, todo!()).await;
        }
        None => {
            error_embed_message(ctx, msg, "データベースに接続できません").await?;
        }
    }

    Ok(())
}
/// 操作の埋め込み
async fn operation_enemy(
    ctx: &serenity::client::Context,
    msg: &Message,
    reactions: [ReactionType; 4],
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| {
                e.title("リアクションを押して操作してね")
                    .description("meme")
            })
            .reactions(reactions.into_iter())
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

/// 結果の埋め込み

/*async fn result_battle(
    ctx: &serenity::client::Context,
    msg: &Message,
    mut playerdata: &CharaConfig,
    mut enemydata: &CharaConfig,
) -> anyhow::Result<BattleState> {
    let vec = turn(&playerdata, &enemydata);
    let mut turn_vec = vec.into_iter().cycle();
    let turn = turn_vec.next().unwrap();

    let player_hp_count = playerdata.charabase.hp;
    let enemy_hp_count = enemydata.charabase.hp;
    let damage = if turn.clone() == playerdata.clone() {
        let mut enemydata_owned = enemydata.to_owned();
        enemydata = calculate_player_damage(&mut enemydata_owned, playerdata);
        enemydata_owned
    } else {
        let mut playerdata_owned = enemydata.to_owned();
        playerdata = calculate_enemy_damage(enemydata, &mut playerdata_owned);
        playerdata_owned
    };

    if player_hp_count > 0 || enemy_hp_count > 0 {
        msg.channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| {
                    e.title(format!(
                        "結果は{}ダメージでした",
                        player_hp_count - damage.charabase.hp
                    ))
                    .description("特記事項はなし！")
                })
            })
            .await
            .context("埋め込みの作成に失敗しました")?;
        Ok(BattleState::BattleContinue)
    } else if player_hp_count <= 0 {
        msg.channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| e.title(format!("{}に倒されてしまった", damage.charabase.name)))
            })
            .await
            .context("埋め込みの作成に失敗しました")?;
        Ok(BattleState::PlayerDown)
    } else if enemy_hp_count <= 0 {
        msg.channel_id
            .send_message(&ctx.http, |f| {
                f.embed(|e| e.title(format!("{}を倒した", damage.charabase.name)))
            })
            .await
            .context("埋め込みの作成に失敗しました")?;
        Ok(BattleState::EnemyDown)
    } else {
        Err(anyhow::anyhow!("何かの不具合が発生しました"))
    }
}*/

async fn guard_attack(
    ctx: &serenity::client::Context,
    msg: &Message,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("{}ダメージをくらった").description("{}%防御した"))
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

async fn error_embed_message<M: Into<String>>(
    ctx: &serenity::client::Context,
    msg: &Message,
    context: M,
) -> Result<Message, anyhow::Error> {
    msg.channel_id
        .send_message(&ctx.http, |f| {
            f.embed(|e| e.title("エラーが発生しました").description(context.into()))
        })
        .await
        .context("埋め込みの作成に失敗しました")
}

/// Battle State
pub enum BattleState {
    BattleContinue,
    PlayerDown,
    EnemyDown,
}
