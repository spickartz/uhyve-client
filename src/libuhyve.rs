extern crate rocket;
#[macro_use]
extern crate serde_json;

mod communicator;
pub mod libuhyve_c;

use communicator::Communicator;
use rocket::http::Status;

const SERVER: &str = "/tmp/uhyve.sock";

/// Starts an application within an empty uhyve container
pub fn start_app(path_to_app: &str) -> Result<Status, Status> {
    let msg = json!({
        "task": "start app",
        "path": path_to_app,
    });
    let status = Communicator::send_cmd(SERVER, &msg.to_string()).unwrap();
    Ok(status)
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
    let status = Communicator::send_cmd(SERVER, &msg.to_string()).unwrap();
    Ok(status)
}

/// Restores a given checkpoint
pub fn load_checkpoint(path_to_checkpoint: &str) -> Result<Status, Status> {
    let msg = json!({
        "task": "load checkpoint",
        "path": path_to_checkpoint,
    });
    let status = Communicator::send_cmd(SERVER, &msg.to_string()).unwrap();
    Ok(status)
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
    let status = Communicator::send_cmd(SERVER, &msg.to_string()).unwrap();
    Ok(status)
}
