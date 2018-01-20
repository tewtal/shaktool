BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS `streams` (
	`id`	INTEGER PRIMARY KEY AUTOINCREMENT,
	`user_id`	TEXT,
	`user_name`	TEXT,
	`game_id`	TEXT,
	`game_name`	TEXT,
	`title`	TEXT,
	`viewers`	TEXT
);
CREATE TABLE IF NOT EXISTS `runners` (
	`id`	INTEGER PRIMARY KEY AUTOINCREMENT,
	`name`	TEXT,
	`dt_id`	TEXT,
	`src_id`	TEXT,
	`sync`	TEXT
);
CREATE TABLE IF NOT EXISTS `records` (
	`id`	INTEGER PRIMARY KEY AUTOINCREMENT,
	`dt_id`	TEXT,
	`src_id`	TEXT,
	`runner_id`	TEXT,
	`category`	TEXT,
	`region`	TEXT,
	`realtime`	TEXT,
	`gametime`	TEXT,
	`comment`	TEXT,
	`video`	TEXT,
	`active`	TEXT,
	`status`	TEXT,
	`sync_status`	TEXT
);
COMMIT;
