-- Your SQL goes here
CREATE TABLE builds (
  id SERIAL PRIMARY KEY,
  source VARCHAR NOT NULL,
  version VARCHAR NOT NULL,
  champion_alias VARCHAR NOT NULL,
  champion_id INTEGER NOT NULL,
  content json NOT NULL
);

CREATE INDEX source_champion ON builds (source, champion_alias, champion_id);
