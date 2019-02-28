#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate futures;
// #![deny(deprecated)]
extern crate tokio;

use clap::{App, SubCommand, Shell};

mod tests;
// use log::Level;
mod code_util;
mod gb18030;
mod table;
mod dir_watcher_stream;
mod real_world;
// mod cli;


use tokio::io;
use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use std::env;
use std::net::SocketAddr;

fn receive_fc_event(addr_str: &str) {
    let addr = addr_str.parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

    let incoming = listener.incoming();

    let server = incoming
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let bytes_copied = tokio::io::copy(reader, writer);
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("I/O error {:?}", err)
            });
            tokio::spawn(handle_conn)
        });
    info!("listening on: {}", addr_str);
    tokio::run(server);
}

// #[cfg(feature = "yaml")]
fn main() {
    ::std::env::set_var("RUST_LOG", "play_rs=debug");
    env_logger::init();

    use clap::App;

    // To load a yaml file containing our CLI definition such as the example '17_yaml.yml' we can
    // use the convenience macro which loads the file at compile relative to the current file
    // similar to how modules are found.
    //
    // Then we pass that yaml object to App to build the CLI.
    //
    // Finally we call get_matches() to start the parsing process. We use the matches just as we
    // normally would
    // let yml = load_yaml!(std::env::current_dir().unwrap().join("17_yaml.yml").to_str().unwrap());
    
    let yml = load_yaml!("17_yaml.yml");
    let m = App::from_yaml(yml).get_matches();

    match m.subcommand() {
        ("completions", Some(sub_matches)) => {
            let shell = sub_matches.value_of("shell");
            // println!("shell value:{:?}", shell);
            let shell_enum = shell.unwrap().parse::<Shell>().unwrap();
            App::from_yaml(yml).gen_completions_to(
                "play-rs", 
                shell_enum, 
                &mut std::io::stdout()
            );
        },
        ("fc-server", Some(sub_matches)) => {
            let mut port = if let Some(inner_port) = sub_matches.value_of("port") {
                inner_port
            } else {
                "8123"
            };
            receive_fc_event(&("127.0.0.1:".to_owned() + port));
        },
        (_, _) => unimplemented!(), // for brevity
    }

    // Because the example 17_yaml.yml is rather large we'll just look a single arg so you can
    // see that it works...
    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _      => unreachable!()
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }

    // // println!("Hello, world!");

    // let op = env::args().nth(1);
    // match &op {
    //     Some(a) => match a.as_str() {
    //         "tokio-server" => {
    //           info!("got parameter: {}", a);
    //           echo().unwrap();
    //         },
    //         "tokio-client" => {
    //             hello_world().unwrap();
    //         },
    //         "clap" => {
    //             info!("run clap.");
    //         	clap();
    //         },
    //         _ => ()
    //     },
    //     _ => {
    //         hello_world().unwrap();
    //         }
    // };
    // hello_world().unwrap();
}