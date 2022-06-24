/// thrpg events
#[derive(Debug, Clone)]
pub enum Events {
    PlayerDefeat,
    EnemyDefeat,
    PlayerWin,
    EnemyWin,
    PlayerSave,
    BOTSetup,
    FightRaid,
    RaidDefeat,
    PlayerStatusCheck,
    PlayerScoreCheck,
    PlayerInfoCheck,
    PlayerSetChera,
    PlayerDataDelete,
    PlayerBattleOperationEmbedReaction(String),
}
