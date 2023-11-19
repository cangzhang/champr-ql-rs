-- Your SQL goes here
CREATE TABLE builds (
  id SERIAL PRIMARY KEY,
  source VARCHAR NOT NULL,
  version VARCHAR NOT NULL,
  champion_alias VARCHAR NOT NULL,
  champion_id VARCHAR NOT NULL,
  content json NOT NULL
);

ALTER TABLE builds ADD UNIQUE (source, champion_id, champion_alias);
