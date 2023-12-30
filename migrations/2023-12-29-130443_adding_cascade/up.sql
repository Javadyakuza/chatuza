ALTER TABLE tron_wallets
DROP CONSTRAINT tron_wallets_user_id_fkey;

ALTER TABLE tron_wallets
ADD CONSTRAINT tron_wallets_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users(user_id)
ON DELETE CASCADE;

ALTER TABLE solana_wallets
DROP CONSTRAINT solana_wallets_user_id_fkey;

ALTER TABLE solana_wallets
ADD CONSTRAINT solana_wallets_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users(user_id)
ON DELETE CASCADE;

ALTER TABLE user_profiles
DROP CONSTRAINT user_profiles_user_id_fkey;

ALTER TABLE user_profiles
ADD CONSTRAINT user_profiles_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users(user_id)
ON DELETE CASCADE;
