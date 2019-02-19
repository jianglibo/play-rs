mod tests;
#[macro_use]
extern crate log;
extern crate env_logger;
// use log::Level;
mod code_util;
mod gb18030;

fn main() {
    ::std::env::set_var("RUST_LOG", "play_rs=debug");
    env_logger::init();
    println!("Hello, world!");
}
