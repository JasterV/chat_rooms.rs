extern crate ws;

use serde_json;
use std::net::SocketAddr;
use std::thread;
use ws::{Handler, Handshake, Message, Result as WResult, Sender as WSender, WebSocket, CloseCode};

struct Room {
    out: WSender
}

impl Room {
    fn open_msg(&self) -> String {
        serde_json::json!({
            "event": "open",
            "id": self.out.connection_id()
        }).to_string()
    }

    fn new_user_msg(&self) -> String {
       serde_json::json!({
            "event": "new_user",
            "id": self.out.connection_id()
        }).to_string()
    }

    fn user_gone_msg(&self) -> String {
        serde_json::json!({
            "event": "user_gone",
            "msg": "A ninja has disconnected" 
        }).to_string()
    }

    fn user_msg(&self, msg: Message) -> String {
        serde_json::json!({
            "event": "message",
            "id": self.out.connection_id(),
            "msg": msg.to_string() 
        }).to_string()
    }
}

impl Handler for Room {
    fn on_open(&mut self, _: Handshake) -> WResult<()> {
        self.out.send(self.open_msg())?;
        self.out.broadcast(self.new_user_msg())?;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WResult<()> {
        Ok(self.out.broadcast(self.user_msg(msg))?)
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.out.broadcast(self.user_gone_msg()).unwrap();
    }
}

pub struct RoomController {
    addr: SocketAddr,
    out: WSender,
}

impl RoomController {
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn shutdown_room(&self) -> WResult<()> {
        Ok(self.out.shutdown()?)
    }
}

pub fn start_room() -> WResult<RoomController> {
    let mut ws = WebSocket::new(|out| Room { out })?;

    ws = ws.bind("127.0.0.1:0").unwrap();
    println!("Room with ip {:?} started", ws.local_addr()?);

    let controller = RoomController {
        addr: ws.local_addr()?,
        out: ws.broadcaster(),
    };

    thread::spawn(move || {
        ws.run().unwrap();
    });

    Ok(controller)
}
