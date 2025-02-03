CREATE TABLE user_api_keys (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    description TEXT NOT NULL,
    key_id TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (key_id)
);

CREATE TABLE applications (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (organization_id, name)
);

CREATE TABLE application_api_keys (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    application_id BIGINT NOT NULL REFERENCES applications(id),
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    description TEXT NOT NULL,
    key_id TEXT NOT NULL,
    key_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    UNIQUE (key_id)
);

CREATE TABLE api_requests (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_api_key_id BIGINT NOT NULL REFERENCES user_api_keys(id),
    application_api_key_id BIGINT NOT NULL REFERENCES application_api_keys(id),
    version INTEGER NOT NULL,
    request_body JSONB,
    response_body JSONB,
    response_status INTEGER,
    duration_ms INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- This function is used to gate sql queries to an organization
CREATE OR REPLACE FUNCTION public.execute_org_query(
    org_id BIGINT,
    query TEXT
) RETURNS SETOF JSON AS $$
BEGIN
    -- Set org context
    PERFORM set_config('app.organization_id', org_id::TEXT, true);

    -- Execute query and return results
    RETURN QUERY EXECUTE query;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- This function is used to gate sql queries to an organization and user
CREATE OR REPLACE FUNCTION public.execute_org_user_query(
    org_id BIGINT,
    user_data TEXT,
    query TEXT
) RETURNS SETOF JSON AS $$
BEGIN
    -- Set org context
    PERFORM set_config('app.organization_id', org_id::TEXT, true);
    PERFORM set_config('app.user_data', user_data, true);

    -- Execute query and return results
    RETURN QUERY EXECUTE query;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
