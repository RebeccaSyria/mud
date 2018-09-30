pub struct Room{
    room_name: String,
    description: String,
    room_id: i32,
    messages: String,

}


pub fn make_room(room_name: String, description: String, room_id: i32, messages: String) -> Room {
    Room{ room_name: room_name, description: description, room_id: room_id, messages: messages}
}

pub fn get_room_description(room: &Room) -> &String{
    let s = &room.description;
    return s;
}
