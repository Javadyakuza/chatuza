ALTER TABLE chat_room_participants
DROP COLUMN room_pub_key;

ALTER TABLE chat_rooms
ADD COLUMN chat_room_pubkey BYTEA NOT NULL;