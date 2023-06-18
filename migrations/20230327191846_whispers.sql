CREATE TABLE IF NOT EXISTS "whispers" (
	"name" TEXT,
	"message" TEXT NOT NULL,
	"private" BOOLEAN NOT NULL,
	"snowflake" BIGINT NOT NULL UNIQUE,
	"timestamp" TEXT NOT NULL,
	PRIMARY KEY ("snowflake")
);