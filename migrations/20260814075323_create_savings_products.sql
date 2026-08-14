-- Add migration script here
CREATE Table savings_products (
    id UUID PRIMARY KEY,

    code VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(150) NOT NULL,
    description TEXT,

    min_amount NUMERIC(20, 2) NOT NULL DEFAULT 0,
    max_amount NUMERIC(20, 2),

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);