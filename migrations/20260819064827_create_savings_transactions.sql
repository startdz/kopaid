-- Add migration script here

CREATE TABLE savings_transactions (
    id UUID PRIMARY KEY,

    savings_id UUID NOT NULL,

    transaction_type VARCHAR(20) NOT NULL,

    amount NUMERIC(20, 2) NOT NULL,

    reference_number VARCHAR(100),
    description TEXT,

    created_by UUID NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_savings_transactions_savings
        FOREIGN KEY (savings_id)
        REFERENCES savings(id)
        ON DELETE RESTRICT,

    CONSTRAINT fk_savings_transactions_user
        FOREIGN KEY (created_by)
        REFERENCES users(id)
        ON DELETE RESTRICT,

    CONSTRAINT chk_savings_transaction_type
        CHECK (
            transaction_type IN ('deposit', 'withdrawal')
        ),

    CONSTRAINT chk_savings_transaction_amount
        CHECK (
            amount > 0
        )
);