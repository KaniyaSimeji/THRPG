# THRPG

分散型の東方RPGゲームを提供するBotです！

## どんなBot

- シンプルな東方のRPGゲーム
- セルフホスト可能
- 分散型でプレイデータなどが別のBotでも大丈夫！(な予定)

## 使用する技術

- Rust
  - serenity
  - SeaORM
- PostgreSQL

## ファイル階層
```
THRPG
|- THRPG
|- THRPG.toml
|- docs
|- chara
|- i18n
   |- Japanese
   |- English
```


### メモ書き

redis-battle-key:

<user-id> {
	player_chara: String,
	enemy_chara: String,
	enemy_damage: u32(String),
	elapesd_turns: u32(String),
	start_time: DataTime(String),
	start_turn: u32(String)
}

`player_chara`はNullではないように(一応Nullか不正な値だと霊夢を選択するようにするが)
`player_chara` からキャラクターの情報(CharaBase)を作れるようにする

`set_chara()`

- キャラクターの名前を引数にして変更出来る
- 引数、良心あるなら半角で空白を打って対応してくれそうだが、全角でもいけるようにしたい(怒りに燃える)
- まずはローマ字。時間があれば日本語でもいけるように。(取れる値はdocsに書いておく)
