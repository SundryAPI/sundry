CREATE TABLE raw (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    organization_id BIGINT REFERENCES organizations(id),
    data_id TEXT NOT NULL,
    source TEXT NOT NULL, -- e.g. github, slack, gsuite
    source_item TEXT NOT NULL, -- e.g. issues, pull request, docs, sheets, channel
    data JSONB NOT NULL,
    UNIQUE (data_id, source, source_item)
);

CREATE TABLE raw_normalized (
    raw_id BIGINT PRIMARY KEY REFERENCES raw(id),
    normalized_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_normalized_at ON raw_normalized(normalized_at);
