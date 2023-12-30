ALTER TABLE solana_wallets
ALTER COLUMN wallet_addr TYPE varchar(60);

ALTER TABLE tron_wallets
ALTER COLUMN wallet_addr TYPE varchar(60);
