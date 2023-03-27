CREATE TABLE "whispers" (
	"name"	    TEXT,
	"message"	TEXT    NOT NULL,
	"private"	BOOLEAN NOT NULL,
	"snowflake"	INTEGER NOT NULL UNIQUE,
	"timestamp"	TEXT    NOT NULL,
	PRIMARY KEY("snowflake")
);