use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::{
    string::FromUtf8Error,
    sync::mpsc::{channel, sync_channel, Receiver, SendError, Sender, SyncSender},
};

pub struct Room {
    rx: Receiver<(String, TcpStream)>,
    tx: Sender<(String, TcpStream)>,
    clients: Vec<TcpStream>,
}

impl Room {
    pub const MAX_CLIENTS: usize = 15;

    pub fn new() -> Self {
        let (tx, rx) = channel();
        Room {
            tx,
            rx,
            clients: vec![],
        }
    }

    pub fn send_broadcast(&mut self, message: String, addr: SocketAddr) {
        self.clients = self
            .clients
            .iter()
            .filter_map(|client| {
                let buff = message.clone().into_bytes();
                if addr == client.local_addr().unwrap() {
                    let mut client = client.try_clone().unwrap();
                    client.write_all(&buff).map(|_| client).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    }

    pub fn start(mut self) -> Result<RoomController, String> {
        match TcpListener::bind("127.0.0.1:0") {
            Ok(listener) => {
                listener
                    .set_nonblocking(true)
                    .expect("Error setting non blocking");
                let (tx, rx) = sync_channel(1);
                let listener = listener.try_clone().unwrap();
                let addr = listener.local_addr().unwrap();
                thread::spawn(move || {
                    self._start(listener, rx);
                });
                Ok(RoomController { addr, closer: tx })
            }
            Err(_) => Err(String::from("Error binding the socket")),
        }
    }

    fn _shutdown(&mut self) {
        self.clients.iter().for_each(|client| {
            client.shutdown(std::net::Shutdown::Both).unwrap();
        });
        self.clients = vec![];
    }

    fn _start(&mut self, listener: TcpListener, closer: Receiver<()>) {
        loop {
            //ON ACCEPT
            if let Ok((mut socket, addr)) = listener.accept() {
                let tx = Sender::clone(&self.tx);
                if self.clients.len() < Room::MAX_CLIENTS {
                    println!("Client {} connected to room!", addr);
                    self.clients.push(socket.try_clone().unwrap());
                    thread::spawn(move || Self::_listen_child(socket, tx));
                } else {
                    socket.write_all(b"The room is full").unwrap();
                }
            }
            //ON CLOSE
            if let Ok(_) = closer.try_recv() {
                self._shutdown();
                break;
            }
            // ON MESSAGE
            if let Ok((msg, socket)) = self.rx.try_recv() {
                self.send_broadcast(msg, socket.local_addr().unwrap());
            }
        }
    }

    fn _listen_child(mut socket: TcpStream, tx: Sender<(String, TcpStream)>) {
        loop {
            let mut buffer = vec![0; 1024];
            match socket.read(&mut buffer) {
                Ok(_) => {
                    if let Ok(message) = Self::parse_request(buffer) {
                        if message.len() <= 0 {
                            break;
                        }
                        tx.send((message, socket.try_clone().unwrap())).unwrap();
                    }
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("closing connection with: {}", socket.local_addr().unwrap());
                    break;
                }
            }
        }
    }

    fn parse_request(req: Vec<u8>) -> Result<String, FromUtf8Error> {
        String::from_utf8(req.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>())
    }
}

pub struct RoomController {
    addr: SocketAddr,
    closer: SyncSender<()>,
}

impl RoomController {
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn shutdown_room(&self) -> Result<(), SendError<()>> {
        Ok(self.closer.send(())?)
    }
}
