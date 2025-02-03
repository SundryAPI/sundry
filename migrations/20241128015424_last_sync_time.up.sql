CREATE TABLE last_sync_time (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    organization_id BIGINT REFERENCES organizations(id),
    source TEXT NOT NULL, -- e.g. github, slack, gsuite
    source_item TEXT, -- e.g. issues, pull request, docs, sheets, channel
    last_sync_time TIMESTAMP,
    UNIQUE (organization_id, source, source_item)
);
