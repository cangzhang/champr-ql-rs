-- Your SQL goes here
CREATE TABLE sources (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    source VARCHAR NOT NULL,
    version VARCHAR NOT NULL
);

CREATE INDEX source_version ON sources (source, version);
