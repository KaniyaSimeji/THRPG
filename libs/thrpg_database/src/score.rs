use crate::redis_connect::AsyncConnection;
use redis::AsyncCommands;

pub struct ScoreData {
    key: String,
    member: String,
    score: u64,
}

pub async fn score_add(connect: &mut AsyncConnection, data: ScoreData) {
    let _: () = connect
        .zadd(data.key, data.member, data.score)
        .await
        .unwrap();
}

pub async fn score_update(connect: &mut AsyncConnection, data: ScoreData) {
    let (_, stop): (u64, u64) = connect
        .zrevrange_withscores(&data.key, 0, 10)
        .await
        .unwrap();

    if data.score >= stop {
        connect
            .zadd(data.key, data.member, data.score)
            .await
            .unwrap()
    }
}
