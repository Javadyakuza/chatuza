CREATE TABLE solana_wallets (
  wallet_id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(user_id),
  pub_key BYTEA NOT NULL,
  wallet_addr BYTEA NOT NULL
);
