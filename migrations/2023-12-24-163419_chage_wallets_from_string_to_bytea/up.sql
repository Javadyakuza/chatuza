ALTER TABLE solana_wallets
ALTER COLUMN wallet_addr TYPE bytea USING wallet_addr::bytea;  -- Convert data to bytea
ALTER TABLE solana_wallets
ALTER COLUMN wallet_addr SET NOT NULL;  -- Enforce non-nullability

ALTER TABLE tron_wallets
ALTER COLUMN wallet_addr TYPE bytea USING wallet_addr::bytea;  -- Convert data to bytea
ALTER TABLE tron_wallets
ALTER COLUMN wallet_addr SET NOT NULL;  -- Enforce non-nullability