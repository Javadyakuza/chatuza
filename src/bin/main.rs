use chatuza_db::add_participant_to_group_chat_room;
use chatuza_db::del_participant_to_group_chat_room;
use chatuza_db::delete_group_chat_room;
use chatuza_db::delete_private_chat_room;
use chatuza_db::diesel::prelude::*;
use chatuza_db::establish_connection;
use chatuza_db::models::*;
use chatuza_db::schema::{
    chat_room_participants::dsl::*, chat_rooms::dsl::*, user_profiles::dsl::*, users::dsl::*,
};
use chatuza_db::update_group_chat_room_info;
use chatuza_db::{add_new_group_chat_room, add_new_p2p_chat_room};
use chatuza_db::{
    add_new_user, delete_user, get_user_with_username, update_user_credits, update_user_profile,
};

fn main() {
    let connection = &mut establish_connection();

    // let mut user_credits = Users {
    //     username: "some_user_3".to_owned(),
    //     email: "some_gmail_3@gmail.com".to_owned(),
    //     password: "@fucker.com003".to_owned(),
    // };

    // let mut _user_profiles = UserProfiles {
    //     user_id: 0,
    //     bio: Some("i code programms like they do me ".to_owned()),
    //     profile_picture: Some("some url".to_owned()),
    // };
    // let userid = get_user_with_username(connection, "some_user_1")
    //     .unwrap()
    //     .user_id;

    // let mut user_profiles2 = UserProfiles {
    //     user_id: userid,
    //     bio: Some("pretty bad motha fucka ".to_owned()),
    //     profile_picture: Some("some url".to_owned()),
    // };

    // let user_credits2 = QUsers {
    //     user_id: userid,
    //     username: "some_user_4".to_owned(),
    //     email: "some_gmail_4@gmail.com".to_owned(),
    //     password: "@fucker.com004".to_owned(),
    // };

    let mut chat_room_num_1_u_1 = ChatRoomParticipants {
        chat_room_id: 0,
        user_id: 4,
        is_admin: false,
    };
    let mut chat_room_num_1_u_2 = ChatRoomParticipants {
        chat_room_id: 0,
        user_id: 2,
        is_admin: false,
    };
    let mut chat_room_num_1_u_3 = ChatRoomParticipants {
        chat_room_id: 0,
        user_id: 5,
        is_admin: false,
    };
    // update_user_credits(connection, &user_credits2);
    // update_user_profile(connection, &user_profiles2);
    // delete_user(connection, userid);
    // add_new_user(connection, &user_credits, &mut user_profiles).unwrap();
    // add_new_p2p_chat_room(
    //     connection,
    //     &mut chat_room_num_1_u_1,
    //     &mut chat_room_num_1_u_2,
    // )
    // .unwrap();

    // delete_private_chat_room(connection, 2, 4).unwrap();
    let chat_room_info = ChatRooms {
        room_name: "silver finders".to_owned(),
        room_description: " a group of dudes searching for gold falls !!".to_owned(),
    };

    // let new_chat_room_id = add_new_group_chat_room(
    //     connection,
    //     &chat_room_info,
    //     5,
    //     vec![&mut chat_room_num_1_u_1],
    // )
    // .unwrap();

    let mut chat_room_num_1_u_2: ChatRoomParticipants = ChatRoomParticipants {
        chat_room_id: 8,
        user_id: 2,
        is_admin: false,
    };
    // .unwrap();
    // let res = update_group_chat_room_info(connection, &"miners".to_owned(), &new_chat_room_info, 5);
    // println!("{:?}", res);

    // add_participant_to_group_chat_room(connection, &chat_room_num_1_u_2).unwrap();
    del_participant_to_group_chat_room(connection, &chat_room_num_1_u_2, 5).unwrap();
    // delete_group_chat_room(connection, &"silver finders".to_owned(), 5).unwrap();
}
