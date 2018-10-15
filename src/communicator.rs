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
        // Connect to server and send json request
        let mut comm = Communicator::connect(server).unwrap();
        comm.sock.write_all(msg.as_bytes()).unwrap();
        comm.sock.shutdown(Shutdown::Write).unwrap();

        // Wait for status code
        let mut status_code = String::new();
        comm.sock.read_to_string(&mut status_code).unwrap();

        // return http status code
        match Status::from_code(status_code.parse().unwrap()) {
            Some(code) => return Ok(code),
            None => return Err(()),
        };
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
}
