#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate uuid;

use lib::rooms::rooms_map::RoomsMap;
use rocket::{
    Rocket,
    http::Status,
    response::status::Custom,
    State,
};
use rocket_contrib::json::Json;
use serde::Serialize;

use uuid::Uuid;

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use lib::cors::options;

#[derive(Serialize)]
struct RoomInfo {
    addr: SocketAddr,
    id: String,
}

#[get("/rooms/<id>")]
fn get_addr(id: String, rooms: State<Arc<Mutex<RoomsMap>>>) -> Result<Json<RoomInfo>, Custom<String>> {
    let arc_clone = _get_state_arc(&rooms);
    let rooms = arc_clone.lock().unwrap();
    match rooms.get_addr(&id) {
        Ok(addr) => Ok(Json(RoomInfo{id, addr})),
        Err(_) => Err(Custom(
            Status::ImATeapot,
            "Can't get the address".to_string(),
        )),
    }
}

#[get("/rooms?create")]
fn create_room(state: State<Arc<Mutex<RoomsMap>>>) -> Result<Json<RoomInfo>, Custom<String>> {
    let arc_clone = _get_state_arc(&state);
    let mut rooms = arc_clone.lock().unwrap();
    let id = Uuid::new_v4().to_string();
    match rooms.start_room(id.clone()) {
        Ok(addr) => {
            println!("Room with {} created!", &id);
            Ok(Json(RoomInfo { addr, id }))
        }
        Err(message) => Err(Custom(Status::Locked, message)),
    }
}

fn _get_state_arc<T: Send + Sync>(state: &State<Arc<Mutex<T>>>) -> Arc<Mutex<T>> {
    let arc = state.inner();
    let arc_clone = Arc::clone(arc);
    arc_clone
}

fn rocket() -> Rocket {
    rocket::ignite()
    .mount("/", routes![get_addr, create_room])
    .manage(Arc::new(Mutex::new(RoomsMap::new())))
    .attach(options())
}

fn main() {
    rocket().launch();
}
