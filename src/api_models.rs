struct NewUserIN {
    credits: Users,
    profile: UserProfiles,
}

struct UpdatedUserCreditsIN {
    user_id: i32,
    username: String,
    email: String,
    password: String,
}

struct UpdatedUserProfileIN {
    user_id: i32,
    bio: String,
    profile_picture: String,
}

// delete user only takes one arg //

// get user with username takes only one argument //

// get user with email takes only one argument //

// get user with user id takes only one argument //

struct NewP2PChatRoomIN {
    requestor_user: i32,
    acceptor_user: i32,
}

struct DeleteP2PChatRoomIN {
    chat_room_id: i32,
    remover_user_id: i32,
}

struct NewGroupChatRoomIN {
    chat_room_info: ChatRooms,
    group_owner_id: i32,
    group_members: Vec<ChatRoomParticipants>, // Note: members should not include the owner
}

struct DeleteGroupChatRoomIN {
    chat_room_name: String,
    remover_user_id: i32,
}

struct UpdatedGroupChatRoomInfoIN {
    old_chat_room_name: String,
    new_chat_room_info: ChatRooms,
    editor_user_id: i32,
}

struct NewGroupChatParticipantIN {
    user_id: i32,
    chat_room_id: i32,
}

struct GroupChatParticipantToRemoveIN {
    removing_user: ChatRoomParticipants,
    remover_user_id: i32,
}

// get functions are getting only one argument
