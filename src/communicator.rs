extern crate rocket;

use rocket::http::Status;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

pub struct Communicator {
    sock: UnixStream,
}

impl Communicator {
    pub fn send_cmd(server: &str, msg: &str) -> Result<Status, ()> {
        let mut comm = Communicator::connect(server).unwrap();
        comm.sock.write_all(msg.as_bytes()).unwrap();

        let mut status_code = String::new();
        comm.sock.read_to_string(&mut status_code).unwrap();
        let res = match Status::from_code(status_code.parse().unwrap()) {
            Some(code) => Ok(code),
            None => Err(()),
        };
        comm.disconnect();

        res
    }
    fn connect(server: &str) -> Result<Communicator, ()> {
        let sock = match UnixStream::connect(server) {
            Ok(sock) => sock,
            Err(e) => {
                println!("Could not connect: {:?}", e);
                return Err(());
            }
        };
        Ok(Communicator { sock: sock })
    }

    fn disconnect(&self) {
        self.sock
            .shutdown(Shutdown::Both)
            .expect("Shutdown of the socket failed!");
    }
}
