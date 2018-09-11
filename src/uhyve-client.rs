extern crate libuhyve;

use libuhyve::uhyve_client;

pub fn main() {
    let res = uhyve_client::create_checkpoint("/foo/bar", true);

    println!("Function returned with {:?}", res)
}
