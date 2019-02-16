mod tests;
#[macro_use]
extern crate log;
extern crate env_logger;
use log::Level;

fn main() {
    ::std::env::set_var("RUST_LOG", "play_rs=debug");
    env_logger::init();
    println!("Hello, world!");
}
