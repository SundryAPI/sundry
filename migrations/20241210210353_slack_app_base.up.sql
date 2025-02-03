INSERT INTO oauth_providers (name) VALUES ('slack');

CREATE SCHEMA slack;

CREATE TABLE slack.app_installations (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    access_token TEXT NOT NULL,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (access_token)
);

CREATE INDEX ON slack.app_installations (organization_id);

-- {
--     "id": "W012A3CDE",
--     "team_id": "T012AB3C4",
--     "name": "spengler",
--     "deleted": false,
--     "color": "9f69e7",
--     "real_name": "spengler",
--     "tz": "America/Los_Angeles",
--     "tz_label": "Pacific Daylight Time",
--     "tz_offset": -25200,
--     "profile": {
--         "avatar_hash": "ge3b51ca72de",
--         "status_text": "Print is dead",
--         "status_emoji": ":books:",
--         "real_name": "Egon Spengler",
--         "display_name": "spengler",
--         "real_name_normalized": "Egon Spengler",
--         "display_name_normalized": "spengler",
--         "email": "spengler@ghostbusters.example.com",
--         "image_24": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "image_32": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "image_48": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "image_72": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "image_192": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "image_512": "https://.../avatar/e3b51ca72dee4ef87916ae2b9240df50.jpg",
--         "team": "T012AB3C4"
--     },
--     "is_admin": true,
--     "is_owner": false,
--     "is_primary_owner": false,
--     "is_restricted": false,
--     "is_ultra_restricted": false,
--     "is_bot": false,
--     "updated": 1502138686,
--     "is_app_user": false,
--     "has_2fa": false
-- }
CREATE TABLE slack.users (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    name TEXT NOT NULL,
    real_name TEXT
);

CREATE INDEX ON slack.users (id);

-- {
--     "id": "C012AB3CD",
--     "name": "general",
--     "is_channel": true,
--     "is_group": false,
--     "is_im": false,
--     "created": 1449252889,
--     "creator": "U012A3CDE",
--     "is_archived": false,
--     "is_general": true,
--     "unlinked": 0,
--     "name_normalized": "general",
--     "is_shared": false,
--     "is_ext_shared": false,
--     "is_org_shared": false,
--     "pending_shared": [],
--     "is_pending_ext_shared": false,
--     "is_member": true,
--     "is_private": false,
--     "is_mpim": false,
--     "updated": 1678229664302,
--     "topic": {
--         "value": "Company-wide announcements and work-based matters",
--         "creator": "",
--         "last_set": 0
--     },
--     "purpose": {
--         "value": "This channel is for team-wide communication and announcements. All team members are in this channel.",
--         "creator": "",
--         "last_set": 0
--     },
--     "previous_names": [],
--     "num_members": 4
-- }
CREATE TABLE slack.channels (
    id TEXT PRIMARY KEY,
    name TEXT,
    name_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', name)) STORED,
    is_channel BOOLEAN NOT NULL DEFAULT TRUE,
    is_group BOOLEAN NOT NULL DEFAULT FALSE,
    is_im BOOLEAN NOT NULL DEFAULT FALSE,
    created BIGINT NOT NULL,
    creator TEXT, -- references slack.users(id),
    "user" TEXT, -- references slack.users(id),
    is_archived BOOLEAN NOT NULL DEFAULT FALSE,
    is_general BOOLEAN NOT NULL DEFAULT FALSE,
    unlinked INTEGER NOT NULL DEFAULT 0,
    name_normalized TEXT,
    is_shared BOOLEAN NOT NULL DEFAULT FALSE,
    is_ext_shared BOOLEAN NOT NULL DEFAULT FALSE,
    is_org_shared BOOLEAN NOT NULL DEFAULT FALSE,
    pending_shared TEXT[] NOT NULL DEFAULT '{}',
    is_pending_ext_shared BOOLEAN NOT NULL DEFAULT FALSE,
    is_member BOOLEAN NOT NULL DEFAULT FALSE,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    is_mpim BOOLEAN NOT NULL DEFAULT FALSE,
    updated BIGINT,
    topic JSONB,
    purpose JSONB,
    previous_names TEXT[] NOT NULL DEFAULT '{}',
    num_members INTEGER
);

CREATE INDEX ON slack.channels USING GIN (name_tsv);

CREATE TABLE slack.app_installation_channels (
    app_installation_id BIGINT NOT NULL REFERENCES organizations(id),
    channel_id TEXT NOT NULL REFERENCES slack.channels(id),
    PRIMARY KEY (app_installation_id, channel_id)
);

CREATE INDEX ON slack.app_installation_channels(app_installation_id, channel_id);

CREATE TABLE slack.channel_members (
    channel_id TEXT NOT NULL, -- references slack.channels(id)
    user_id TEXT NOT NULL, -- references slack.users(id)
    PRIMARY KEY (channel_id, user_id)
);

CREATE INDEX ON slack.channel_members(channel_id);
CREATE INDEX ON slack.channel_members(user_id);

-- {
--     "type": "message",
--     "user": "U123ABC456",
--     "text": "Shutters shut and shutters and so shutters shut and shutters and so and so shutters and so shutters shut and so shutters shut and shutters and so.",
--     "thread_ts": "1482960137.003543",
--     "parent_user_id": "U123ABC456",
--     "ts": "1483037603.017503"
-- }
CREATE TABLE slack.messages (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY, -- slack messages do not have ids
    channel_id TEXT NOT NULL, -- REFERENCES slack.channels(id)
    "type" TEXT NOT NULL DEFAULT 'message',
    "user" TEXT, -- REFERENCES slack.users(id),
    text TEXT,
    text_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', COALESCE(text, ''))) STORED,
    thread_ts TEXT,
    parent_user_id TEXT, -- REFERENCES slack.users(id),
    ts TEXT NOT NULL,
    UNIQUE (channel_id, ts) -- time stamps are supposed to be guaranteed to be unique in a channel
);

CREATE INDEX ON slack.messages (thread_ts);
CREATE INDEX ON slack.messages (ts);
CREATE INDEX ON slack.messages USING GIN (text_tsv);

-- Our views

CREATE VIEW slack.org_user_channels AS
SELECT c.*
FROM slack.channels c
JOIN slack.app_installation_channels aic ON c.id = aic.channel_id
JOIN slack.app_installations ai ON aic.app_installation_id = ai.id
JOIN slack.channel_members cm ON c.id = cm.channel_id
WHERE 
    ai.organization_id = current_setting('app.organization_id')::integer
    AND cm.user_id = current_setting('app.user_data');

CREATE VIEW slack.org_user_channel_members AS
SELECT cm.*
FROM slack.channel_members cm
WHERE cm.channel_id IN (
    SELECT id FROM slack.org_user_channels
);

CREATE VIEW slack.org_user_users AS
SELECT u.*
FROM slack.users u
WHERE u.id IN (
    SELECT user_id from slack.org_user_channel_members
);

CREATE VIEW slack.org_user_messages AS
SELECT m.*
FROM slack.messages m
WHERE channel_id IN (
    SELECT id FROM slack.org_user_channels
);
