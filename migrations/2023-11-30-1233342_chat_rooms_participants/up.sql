CREATE TABLE chat_room_participants (
    participant_id SERIAL PRIMARY KEY,
    chat_room_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (chat_room_id) REFERENCES chat_rooms(chat_room_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
