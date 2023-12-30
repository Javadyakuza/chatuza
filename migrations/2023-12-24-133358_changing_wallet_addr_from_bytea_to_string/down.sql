ALTER TABLE tron_wallets DROP COLUMN wallet_addr;
ALTER TABLE tron_wallets ADD COLUMN wallet_addr bytea;

ALTER TABLE solana_wallets DROP COLUMN wallet_addr;
ALTER TABLE solana_wallets ADD COLUMN wallet_addr bytea;
