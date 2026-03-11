-- Add migration script here
-- Create the Products table using UUIDs
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create the Inventory Variants table
CREATE TABLE inventory_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    size VARCHAR(10) NOT NULL,
    stock_quantity INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure only valid sizes are allowed
    CONSTRAINT valid_size CHECK (size IN ('XS', 'SM', 'M', 'L')),
    
    -- Prevent duplicate sizes for the same product
    CONSTRAINT unique_product_size UNIQUE (product_id, size)
);