use crate::error::Result;
use crate::helpers::date::{DateTime, Local};

use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
pub use tungstenite::Message;
use tungstenite::{connect, WebSocket as Ws};

#[derive(Debug)]
pub enum MessageType {
    Login,
    GetInstrumentPrice,
    Other,
}

#[derive(Debug)]
pub struct WebSocket {
    socket: Ws<MaybeTlsStream<TcpStream>>,
}

impl WebSocket {
    pub async fn connect(url: &str) -> Self {
        let (mut socket, response) = connect(url).expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        Self { socket }
    }

    pub async fn send(&mut self, msg: &str) -> Result<()> {
        println!("[Sending]: {}", msg);
        self.socket.write_message(Message::text(msg)).unwrap();
        Ok(())
        // self.socket.read_message().expect("Error reading message")
    }

    pub async fn read(&mut self) -> Result<Message> {
        let msg = self.socket.read_message().expect("Error reading message");
        Ok(msg)
    }
}
