INSERT INTO oauth_providers (name) VALUES ('github');

CREATE SCHEMA github;

CREATE TABLE github.app_installations (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    installation_id BIGINT NOT NULL,
    organization_id BIGINT REFERENCES organizations(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (installation_id)
);

-- {
--   "owner": {
--     "id": 1,
--     "login": "octocat"
--   }
-- }
CREATE TABLE github.users (
    id BIGINT PRIMARY KEY, -- id
    login TEXT NOT NULL -- login
);

-- {
--   "total_count": 1,
--   "repositories": [
--     {
--       "id": 1296269,
--       "full_name": "octocat/Hello-World",
--       "owner": {
--         "login": "octocat",
--         "id": 1,
--       },
--       "private": false,
--       "url": "https://api.github.com/repos/octocat/Hello-World"
--     }
--   ]
-- }
CREATE TABLE github.repositories (
    id BIGINT PRIMARY KEY, -- id
    full_name TEXT NOT NULL, -- full_name
    owner_id BIGINT NOT NULL REFERENCES github.users(id), -- owner.id
    private BOOLEAN NOT NULL, -- private
    url TEXT NOT NULL -- url
);

CREATE TABLE github.app_installation_repositories (
    app_installation_id BIGINT NOT NULL REFERENCES organizations(id),
    repository_id BIGINT NOT NULL REFERENCES github.repositories(id),
    PRIMARY KEY (app_installation_id, repository_id)
);

-- [
--   {
--     "id": 1,
--     "url": "https://api.github.com/repos/octocat/Hello-World/issues/1347",
--     "comments_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347/comments",
--     "number": 1347,
--     "state": "open",
--     "title": "Found a bug",
--     "body": "I'm having a problem with this.",
--     "user": {
--       "id": 1,
--     },
--     "labels": [
--       {
--         "id": 208045946,
--         "name": "bug",
--         "description": "Something isn't working",
--       }
--     ],
--     "assignee": {
--       "id": 1,
--     },
--     "assignees": [
--       {
--         "id": 1,
--       }
--     ],
--     "pull_request": {
--       "url": "https://api.github.com/repos/octocat/Hello-World/pulls/1347",
--     },
--     "closed_at": null,
--     "created_at": "2011-04-22T13:33:48Z",
--     "updated_at": "2011-04-22T13:33:48Z",
--     "closed_by": {
--       "id": 1,
--     }
--   }
-- ]
CREATE TABLE github.issues (
    id BIGINT PRIMARY KEY, -- id
    repository_id BIGINT REFERENCES github.repositories(id),
    url TEXT NOT NULL, -- url
    comments_url TEXT NOT NULL, -- comments_url
    number INTEGER NOT NULL, -- number
    state TEXT NOT NULL, -- state
    title TEXT NOT NULL, -- title
    title_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', title)) STORED,
    body TEXT, -- body
    body_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', COALESCE(body, ''))) STORED,
    user_id BIGINT NOT NULL REFERENCES github.users(id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    closed_at TIMESTAMP,
    closed_by BIGINT REFERENCES github.users(id),
    UNIQUE (repository_id, number)
);

CREATE INDEX ON github.issues USING GIN (title_tsv);
CREATE INDEX ON github.issues USING GIN (body_tsv);

CREATE TABLE github.issue_assignees (
    issue_id BIGINT REFERENCES github.issues(id),
    user_id BIGINT REFERENCES github.users(id),
    PRIMARY KEY (issue_id, user_id)
);

CREATE TABLE github.pull_requests (
    id BIGINT UNIQUE REFERENCES github.issues(id),
    merged_at TIMESTAMP
);

-- [
--   {
--     "id": 1,
--     "url": "https://api.github.com/repos/octocat/Hello-World/issues/comments/1",
--     "body": "Me too",
--     "user": {
--       "id": 1,
--     },
--     "created_at": "2011-04-14T16:00:49Z",
--     "updated_at": "2011-04-14T16:00:49Z",
--   }
-- ]
CREATE TABLE github.comments (
    id BIGINT PRIMARY KEY,
    issue_id BIGINT REFERENCES github.issues(id),
    user_id BIGINT REFERENCES github.users(id),
    url TEXT NOT NULL, -- url
    body TEXT NOT NULL, -- body
    body_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', COALESCE(body,''))) STORED,
    created_at TIMESTAMP NOT NULL, -- created_at
    updated_at TIMESTAMP NOT NULL -- updated_at
);

CREATE INDEX ON github.comments USING GIN (body_tsv);

-- {
--   "id": 208045946,
--   "name": "bug",
--   "description": "Something isn't working",
-- }
CREATE TABLE github.labels (
    id BIGINT PRIMARY KEY, -- id
    name TEXT NOT NULL, -- name
    description TEXT -- description
);

CREATE TABLE github.issue_labels (
    issue_id BIGINT REFERENCES github.issues(id),
    label_id BIGINT REFERENCES github.labels(id),
    PRIMARY KEY (issue_id, label_id)
);

-- Our views and the indexes to make using them fast

CREATE INDEX ON github.app_installations(organization_id);
CREATE INDEX ON github.app_installation_repositories(app_installation_id, repository_id);
CREATE INDEX ON github.issues(repository_id);
CREATE INDEX ON github.comments(issue_id);
CREATE INDEX ON github.issue_labels(issue_id);
CREATE INDEX ON github.issue_assignees(issue_id);

-- Views
CREATE VIEW github.org_repositories AS
SELECT r.*
FROM github.repositories r
JOIN github.app_installation_repositories air ON r.id = air.repository_id
JOIN github.app_installations ai ON air.app_installation_id = ai.id
WHERE ai.organization_id = current_setting('app.organization_id')::integer;

CREATE VIEW github.org_issues AS
SELECT i.*
FROM github.issues i
WHERE i.repository_id IN (
    SELECT id FROM github.org_repositories
);

CREATE VIEW github.org_comments AS
SELECT c.*
FROM github.comments c
WHERE c.issue_id IN (
    SELECT i.id
    FROM github.issues i
    WHERE i.repository_id IN (
        SELECT id FROM github.org_repositories
    )
);

CREATE VIEW github.org_pull_requests AS
SELECT pr.*
FROM github.pull_requests pr
WHERE pr.id IN (
    SELECT i.id
    FROM github.issues i
    WHERE i.repository_id IN (
        SELECT id FROM github.org_repositories
    )
);

CREATE VIEW github.org_issue_labels AS
SELECT il.*
FROM github.issue_labels il
WHERE il.issue_id IN (
    SELECT i.id
    FROM github.issues i
    WHERE i.repository_id IN (
        SELECT id FROM github.org_repositories
    )
);

CREATE VIEW github.org_issue_assignees AS
SELECT ia.*
FROM github.issue_assignees ia
WHERE ia.issue_id IN (
    SELECT i.id
    FROM github.issues i
    WHERE i.repository_id IN (
        SELECT id FROM github.org_repositories
    )
);
