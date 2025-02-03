#!/bin/bash
#
# Create a test user

# Read the .env file
source .env

# Check if DEV_MODE is true
if [ "$PUBLIC_DEV_MODE" != "true" ]; then
    echo "Error: DEV_MODE is not set to true"
    exit 1
fi

# Function to run a SQL query
run_query() {
    psql "$DATABASE_URL" -c "$1"
}

# Run queries
run_query "
  INSERT INTO users (email, password_hash) VALUES ('test@test.com', '\$argon2id\$v=19\$m=19456,t=2,p=1\$rfIq2TVwliUZl+OtFsvo8A\$ab02SLwvPvaGmlHhSaWWZKo5tPJPhMiNbxT0+OYyyDw');

  INSERT INTO organizations (name) VALUES ('Personal Test');

  INSERT INTO user_organizations (user_id, organization_id, role)
  VALUES (
    (SELECT id FROM users WHERE email = 'test@test.com'),
    (SELECT id FROM organizations WHERE name = 'Personal Test'),
    'owner'
  );
"
