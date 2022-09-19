use crate::redis_connect::AsyncConnection;
use redis::AsyncCommands;

#[derive(Debug,Clone)]
pub struct ScoreData {
    key: String,
    member: String,
    score: u64,
}

pub async fn score_add(connect: &mut AsyncConnection, data: ScoreData) -> anyhow::Result<()> {
    let _: () = connect
        .zadd(data.key, data.member, data.score)
        .await?;
    Ok(())
}

pub async fn score_update(connect: &mut AsyncConnection, data: ScoreData) -> anyhow::Result<()> {
    let (_, stop): (u64, u64) = connect
        .zrevrange_withscores(&data.key, 0, 10)
        .await?;

    if data.score >= stop {
        connect
            .zadd(data.key, data.member, data.score)
            .await?;
    }

    Ok(())
}
