CREATE TABLE IF NOT EXISTS orders (
  id UUID PRIMARY KEY,
  customer_name TEXT NOT NULL,
  item TEXT NOT NULL,
  quantity INTEGER NOT NULL CHECK (quantity > 0),
  price_cents BIGINT NOT NULL CHECK (price_cents >= 0),
  status VARCHAR(20) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
