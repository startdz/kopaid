-- Add migration script here
CREATE Table members (
    id UUID PRIMARY KEY,
    member_number VARCHAR(50) NOT NULL UNIQUE,

    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(150),
    phone VARCHAR(50),

    address TEXT,

    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);