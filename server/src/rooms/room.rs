extern crate ws;

use serde_json;
use std::net::SocketAddr;
use std::thread;
use ws::{Handler, Handshake, Message, Result as WResult, Sender as WSender, WebSocket};

struct Room {
    out: WSender,
}

impl Handler for Room {
    fn on_open(&mut self, _: Handshake) -> WResult<()> {
        let response = serde_json::json!({
            "first": true,
            "id": self.out.connection_id()
        });
        self.out.send(response.to_string())?;
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> WResult<()> {
        let response = serde_json::json!({
            "first": false,
            "id": self.out.connection_id(),
            "msg": msg.to_string() 
        });
        Ok(self.out.broadcast(response.to_string())?)
    }

    fn on_shutdown(&mut self) {
        println!(
            "Room with sender id {} closing...",
            self.out.connection_id()
        )
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
