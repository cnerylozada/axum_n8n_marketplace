CREATE TABLE employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    days_off_allowance INT NOT NULL DEFAULT 0,
    days_off_used INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
