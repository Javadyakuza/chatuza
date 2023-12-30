CREATE TABLE chat_rooms (
    chat_room_id SERIAL PRIMARY KEY,
    room_name VARCHAR(255) UNIQUE NOT NULL,
    room_description VARCHAR(255)
);
