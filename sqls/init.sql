/*
PostgreSQL's SQL file
*/
CREATE DATABASE thrpg;

\connect thrpg

CREATE TABLE userdata (
	user_id 	text 	NOT NULL PRIMARY KEY,
	player 		text,
	level 		bigint,
	exp 		bigint,
	battle_uuid Uuid
);

CREATE TABLE playdata (
	battle_uuid 	Uuid 	NOT NULL PRIMARY KEY
    player 		text
    enemy 		text
    elapesd_turns 	bigint
    start_time 	DataTime
    start_turn 	bigint
);
