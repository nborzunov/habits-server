CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR,
    email VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    bio TEXT,
    image VARCHAR,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE habits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR NOT NULL,
    periodicity VARCHAR NOT NULL,
    periodicity_value TEXT[],
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    goal INTEGER NOT NULL,
    goal_type VARCHAR NOT NULL,
    allow_skip BOOLEAN NOT NULL,
    allow_partial_completion BOOLEAN NOT NULL,
    allow_over_goal_completion BOOLEAN NOT NULL,
    can_be_finished BOOLEAN NOT NULL,
    total_goal INTEGER NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE targets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    habit_id UUID NOT NULL REFERENCES habits(id),
    user_id UUID NOT NULL REFERENCES users(id),
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    target_type VARCHAR NOT NULL,
    value INTEGER NOT NULL,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    a_order INTEGER NOT NULL,
    key VARCHAR NOT NULL,
    achievement_type VARCHAR NOT NULL,
    completed_date TIMESTAMP WITH TIME ZONE,
    completed BOOLEAN NOT NULL,
    progress INTEGER NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR NOT NULL,
    currency VARCHAR NOT NULL,
    account_type VARCHAR NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    category_type VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    icon VARCHAR NOT NULL,
    color VARCHAR NOT NULL,
    is_default BOOLEAN NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP WITH TIME ZONE
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    account_id UUID NOT NULL REFERENCES accounts(id),
    category_id UUID NOT NULL REFERENCES categories(id),
    transaction_type VARCHAR NOT NULL,
    note TEXT,
    amount DOUBLE PRECISION NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    deleted BOOLEAN NOT NULL DEFAULT FALSE
);