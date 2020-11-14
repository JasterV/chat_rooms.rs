extern crate ws;

use std::net::SocketAddr;
use std::thread;
use ws::{Sender as WSender, Handler, Result as WResult, WebSocket, Message};

struct Room {
    out: WSender
}

impl Handler for Room {
    fn on_message(&mut self, msg: Message) -> WResult<()> {
        Ok(self.out.broadcast(msg)?)
    }

    fn on_shutdown(&mut self) {
        println!("Room with sender id {} closing...", self.out.connection_id())
    }
}

pub struct RoomController {
    addr: SocketAddr,
    out: WSender
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
    let mut ws = WebSocket::new(|out| {
        Room {out}
    })?;

    ws = ws.bind("127.0.0.1:0").unwrap();
    println!("Room with ip {:?} started", ws.local_addr()?);

    let controller = RoomController {
        addr: ws.local_addr()?,
        out: ws.broadcaster()
    };

    thread::spawn(move || {
        ws.run().unwrap();
    });

    Ok(controller)
}
