use std::collections::HashMap;
use std::io::{self, ErrorKind};
use std::net::{SocketAddr};
use super::room::{start_room, RoomController};

pub struct RoomsMap(HashMap<String, RoomController>);

impl RoomsMap {
    pub const MAX_ROOMS: usize = 10;
    pub const ROOMS_TIMEOUT: u64 = 60 * 10; // timeout in seconds

    pub fn new() -> Self {
        RoomsMap(HashMap::new())
    }

    pub fn get_addr(&self, id: &str) -> io::Result<SocketAddr> {
        match self.0.get(id) {
            Some(controller) => Ok(controller.addr()),
            None => Err(io::Error::new(ErrorKind::AddrNotAvailable, ":(")),
        }
    }

    pub fn start_room(&mut self, id: String) -> Result<SocketAddr, String> {
        if self.0.contains_key(&id) {
            Err(String::from("Room already exists"))
        } else if self.0.len() >= Self::MAX_ROOMS {
            Err(format!("Can't create more than {} rooms", Self::MAX_ROOMS))
        } else {
            match start_room() {
                Ok(controller) => {
                    let addr = controller.addr();
                    self.0.insert(id, controller);
                    Ok(addr)
                }
                Err(_) => Err(String::from("There was an error starting a room")),
            }
        }
    }

    pub fn close_room(&mut self, id: String) -> Result<(), String> {
        match self.0.get(&id) {
            Some(controller) => {
                match controller.shutdown_room() {
                    Ok(_) => {
                        self.0.remove(&id);
                        Ok(())
                    },
                    Err(_) => Err(String::from("Can't close the room"))
                }
            }
            None => Err(String::from("The room doesnt exists")),
        }
    }
}

