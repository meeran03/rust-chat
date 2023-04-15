use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct ChatRoom {
    clients: Mutex<HashMap<String, TcpStream>>,
}

impl ChatRoom {
    pub fn new() -> Arc<Self> {
        Arc::new(ChatRoom {
            clients: Mutex::new(HashMap::new()),
        })
    }

    pub fn join(&self, nickname: String, mut stream: TcpStream) {
        let mut clients = self.clients.lock().unwrap();
        writeln!(stream, "Welcome to the chat, {}!", nickname).unwrap();
        clients.insert(nickname.clone(), stream.try_clone().unwrap());
        drop(clients);

        let mut buffer = vec![0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    self.leave(&nickname);
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                    self.broadcast(&nickname, &msg);
                }
                Err(_) => {
                    self.leave(&nickname);
                    break;
                }
            }
        }
    }

    fn leave(&self, nickname: &str) {
        let mut clients = self.clients.lock().unwrap();
        clients.remove(nickname);
        self.broadcast(nickname, "has left the chat.");
    }

    fn broadcast(&self, sender: &str, message: &str) {
        let clients = self.clients.lock().unwrap();
        for (nickname, client) in clients.iter() {
            if nickname.as_str() != sender {
                // writeln!(client, "{}: {}", sender, message).unwrap();
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, chat_room: Arc<ChatRoom>) {
    let mut nickname_buffer = vec![0; 64];
    stream.read(&mut nickname_buffer).unwrap();
    let nickname = String::from_utf8_lossy(&nickname_buffer).trim_end().to_string();
    chat_room.join(nickname, stream);
}
