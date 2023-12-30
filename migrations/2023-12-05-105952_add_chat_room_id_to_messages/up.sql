ALTER TABLE messages
ADD COLUMN chat_room_id INTEGER NOT NULL REFERENCES chat_rooms(chat_room_id);