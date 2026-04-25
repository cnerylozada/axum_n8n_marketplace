CREATE TYPE time_off_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE time_off_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    status time_off_status NOT NULL DEFAULT 'pending',
    days INT NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    finish_date TIMESTAMPTZ NOT NULL
);
