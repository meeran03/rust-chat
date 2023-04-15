use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

mod chat_room;

use chat_room::ChatRoom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    println!("Chat server started on port {}", port);

    let chat_room = Arc::new(ChatRoom::new());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let chat_room_clone = Arc::clone(&chat_room);
        thread::spawn(move || {
            handle_client(stream, chat_room_clone);
        });
    }
}

fn handle_client(mut stream: TcpStream, chat_room: Arc<ChatRoom>) {
    let mut nickname_buffer = vec![0; 64];
    stream.read(&mut nickname_buffer).unwrap();
    let nickname = String::from_utf8_lossy(&nickname_buffer).trim_end().to_string();
    chat_room.join(nickname, stream);
}
