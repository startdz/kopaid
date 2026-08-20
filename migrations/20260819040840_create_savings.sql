-- Add migration script here
CREATE SEQUENCE savings_account_number_seq START 1;

CREATE TABLE savings (
    id UUID PRIMARY KEY,

    account_number VARCHAR(50) NOT NULL UNIQUE,

    member_id UUID NOT NULL,
    product_id UUID NOT NULL,

    balance NUMERIC(20, 2) NOT NULL DEFAULT 0,

    is_active BOOLEAN NOT NULL DEFAULT TRUE,

    opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_savings_member
        FOREIGN KEY (member_id)
        REFERENCES members(id)
        ON DELETE RESTRICT,

    CONSTRAINT fk_savings_product
        FOREIGN KEY (product_id)
        REFERENCES savings_products(id)
        ON DELETE RESTRICT,
    
    CONSTRAINT uq_savings_member_product
        UNIQUE (member_id, product_id)
);

-- Satu member hanya boleh memiliki satu account untuk satu produk simpanan.