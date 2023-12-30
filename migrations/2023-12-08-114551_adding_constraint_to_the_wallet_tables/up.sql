ALTER TABLE solana_wallets
ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id);


ALTER TABLE tron_wallets
ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id);