// @generated automatically by Diesel CLI.

diesel::table! {
    chat_room_participants (participant_id) {
        participant_id -> Int4,
        chat_room_id -> Int4,
        user_id -> Int4,
        is_admin -> Bool,
    }
}

diesel::table! {
    chat_rooms (chat_room_id) {
        chat_room_id -> Int4,
        #[max_length = 255]
        room_name -> Varchar,
        #[max_length = 255]
        room_description -> Varchar,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        sender_id -> Int4,
        recipient_id -> Int4,
        timestamp -> Timestamp,
        content -> Text,
        is_read -> Bool,
        #[max_length = 255]
        delivery_status -> Varchar,
        parent_message_id -> Nullable<Int4>,
        chat_room_id -> Int4,
    }
}

diesel::table! {
    user_profiles (user_profile_id) {
        user_profile_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        bio -> Nullable<Varchar>,
        #[max_length = 255]
        profile_picture -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(chat_room_participants -> chat_rooms (chat_room_id));
diesel::joinable!(chat_room_participants -> users (user_id));
diesel::joinable!(messages -> chat_rooms (chat_room_id));
diesel::joinable!(user_profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_room_participants,
    chat_rooms,
    messages,
    user_profiles,
    users,
);
