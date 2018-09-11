extern crate rocket;
#[macro_use]
extern crate serde_json;

// ============= C API ============================================================================
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start_app(path_to_app_ptr: *const c_char) -> u16 {
    let path_to_app = unsafe { CStr::from_ptr(path_to_app_ptr) };

    match path_to_app.to_str() {
        Ok(str) => uhyve_client::start_app(str).unwrap().code,
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn create_checkpoint(
    path_to_checkpoint_ptr: *const c_char,
    full_checkpoint: bool,
) -> u16 {
    let path_to_checkpoint = unsafe { CStr::from_ptr(path_to_checkpoint_ptr) };

    match path_to_checkpoint.to_str() {
        Ok(str) => {
            uhyve_client::create_checkpoint(str, full_checkpoint)
                .unwrap()
                .code
        }
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn load_checkpoint(path_to_checkpoint_ptr: *const c_char) -> u16 {
    let path_to_checkpoint = unsafe { CStr::from_ptr(path_to_checkpoint_ptr) };

    match path_to_checkpoint.to_str() {
        Ok(str) => uhyve_client::load_checkpoint(str).unwrap().code,
        Err(_) => 400,
    }
}

#[no_mangle]
pub extern "C" fn migrate(
    destination: *const c_char,
    mig_type: *const c_char,
    mig_mode: *const c_char,
    use_odp: u8,
    prefetch: u8,
) -> u16 {
    let destination = unsafe { CStr::from_ptr(destination) }.to_str().unwrap();
    let mig_type = unsafe { CStr::from_ptr(mig_type) }.to_str().unwrap();
    let mig_mode = unsafe { CStr::from_ptr(mig_mode) }.to_str().unwrap();
    let use_odp = use_odp == 1;
    let prefetch = prefetch == 1;
    uhyve_client::migrate(destination, mig_type, mig_mode, use_odp, prefetch)
        .unwrap()
        .code
}
// ================================================================================================

pub mod uhyve_client {
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

    /// Starts an application within an empty uhyve container
    pub fn start_app(path_to_app: &str) -> Result<Status, Status> {
        let msg = json!({
            "task": "start app",
            "path": path_to_app,
        });
        let comm = Communicator::send_cmd("/tmp/uhyve.sock", &msg.to_string()).unwrap();
        Ok(Status::Ok)
    }

    /// Triggers the creation of a checkpoint
    pub fn create_checkpoint(
        path_to_checkpoint: &str,
        full_checkpoint: bool,
    ) -> Result<Status, Status> {
        let msg = json!({
            "task": "create checkpoint",
            "params": {
                "path": path_to_checkpoint,
                "full-checkpoint": full_checkpoint,
            }
        });
        let comm = Communicator::send_cmd("/tmp/uhyve.sock", &msg.to_string()).unwrap();
        Ok(Status::Ok)
    }

    /// Restores a given checkpoint
    pub fn load_checkpoint(path_to_checkpoint: &str) -> Result<Status, Status> {
        let msg = json!({
            "task": "load checkpoint",
            "path": path_to_checkpoint,
        });
        let comm = Communicator::send_cmd("/tmp/uhyve.sock", &msg.to_string()).unwrap();
        Ok(Status::Ok)
    }

    /// Migrates a uhyve instance to another node
    /// # Arguments
    /// * `destination` - A String slice that holds the IP address of the destination
    /// * `mig_type` - A string slice specifying the migration type
    /// * `mig_mode` - A string slice specifying the migration mode
    /// * `use_odp` - A boolean whether to enable ODP
    /// * `prefetch` - A boolean whether to enable prefetch (only used in conjunction with ODP)
    pub fn migrate(
        destination: &str,
        mig_type: &str,
        mig_mode: &str,
        use_odp: bool,
        prefetch: bool,
    ) -> Result<Status, Status> {
        let msg = json!({
            "task": "migrate",
            "params": {
                "destination": destination,
                "type": mig_type,
                "mode": mig_mode,
                "use-odp": use_odp,
                "prefetch": prefetch,
            },
        });
        let comm = Communicator::send_cmd("/tmp/uhyve.sock", &msg.to_string()).unwrap();
        Ok(Status::Ok)
    }
}
