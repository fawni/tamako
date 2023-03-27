CREATE TABLE "whispers" (
	"name"	    TEXT,
	"messeage"	TEXT    NOT NULL,
	"private"	INTEGER NOT NULL CHECK(private = 0 OR private = 1),
	"snowflake"	INTEGER NOT NULL UNIQUE,
	"timestamp"	TEXT    NOT NULL,
	PRIMARY KEY("snowflake")
);