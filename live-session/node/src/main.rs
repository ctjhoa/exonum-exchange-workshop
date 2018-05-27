extern crate dotenv;
#[macro_use]
extern crate log;
extern crate exonum;

fn main() {
    dotenv::dotenv().ok();
    exonum::helpers::init_logger().unwrap();
    info!("RustFrest Cryptoexchange");
    println!("Hello, world!");
}
