// @generated automatically by Diesel CLI.

diesel::table! {
    chat_room_participants (participant_id) {
        participant_id -> Int4,
        chat_room_id -> Int4,
        user_id -> Int4,
        is_admin -> Bool,
        room_pub_key -> Bytea,
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
    solana_wallets (wallet_id) {
        wallet_id -> Int4,
        user_id -> Int4,
        wallet_backup -> Bytea,
        wallet_addr -> Bytea,
    }
}

diesel::table! {
    tron_wallets (wallet_id) {
        wallet_id -> Int4,
        user_id -> Int4,
        wallet_backup -> Bytea,
        wallet_addr -> Bytea,
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
diesel::joinable!(user_profiles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_room_participants,
    chat_rooms,
    solana_wallets,
    tron_wallets,
    user_profiles,
    users,
);
