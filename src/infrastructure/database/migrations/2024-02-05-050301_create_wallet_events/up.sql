BEGIN TRANSACTION;
	CREATE TYPE IF NOT EXISTS WalletEvent AS ENUM ('Created');

	-- Your SQL goes here
	CREATE TABLE wallet_events IF NOT EXISTS (
		id INTEGER PRIMARY KEY AUTO,
		wallet_id VARCHAR(1024),
		event_type WalletEvent 
	)
COMMIT;