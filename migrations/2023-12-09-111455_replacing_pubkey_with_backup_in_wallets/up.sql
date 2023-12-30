ALTER TABLE solana_wallets
DROP COLUMN pub_key,
ADD COLUMN wallet_backup BYTEA NOT NULL;

ALTER TABLE tron_wallets
DROP COLUMN pub_key,
ADD COLUMN wallet_backup BYTEA NOT NULL;