extern crate rocket;

use rocket::http::Status;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

pub fn send_cmd(server: &str, msg: &str) -> Result<Status, Status> {
    // Connect to server and send json request
    let mut sock = match UnixStream::connect(server) {
        Ok(sock) => sock,
        Err(e) => {
            println!(
                "[ERROR] Could not connect to '{}' ({:?}). Abort!",
                server, e
            );
            return Err(Status::from_code("503".parse().unwrap()).unwrap());
        }
    };

    sock.write_all(msg.as_bytes()).unwrap();
    sock.shutdown(Shutdown::Write).unwrap();

    // Wait for status code
    let mut status_code = String::new();
    sock.read_to_string(&mut status_code).unwrap();

    // return http status code
    match Status::from_code(status_code.parse().unwrap()) {
        Some(code) => return Ok(code),
        None => return Err(Status::from_code("500".parse().unwrap()).unwrap()),
    };
}
