extern crate libuhyve;

pub fn main() {
    let res = libuhyve::create_checkpoint("/foo/bar", true);

    println!("Function returned with {:?}", res)
}
