use std::net::Shutdown;
use std::os::unix::net::UnixStream;

static UHYVE_SOCK: &'static str = "/tmp/uhyve.sock";
pub struct Communicator {
    server_str: String,
    sock: UnixStream,
}

impl Communicator {
    pub fn new(server: &str) -> Communicator {
        Communicator { server.to_string() }
    }

    pub fn connect(&self) {
        self.sock = match UnixStream::connect("/tmp/uhyve.sock") {
            Ok(sock) => sock,
            Err(e) => {
                println!("Could not connect: {:?}", e);
                return;
            }
        }
    }

    pub fn disconnect(&self) {
        self.sock
            .shutdown(Shutdown::Both)
            .expect("Shutdown of the socket failed!");
    }
}
