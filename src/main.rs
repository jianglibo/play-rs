#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

use clap::{App, SubCommand, Shell};

mod tests;
// use log::Level;
mod code_util;
mod gb18030;
mod table;
// mod cli;

// #![deny(deprecated)]
extern crate tokio;

use tokio::io;
use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use std::env;
use std::net::SocketAddr;

const PORT: &str = "127.0.0.1:6142";

fn echo() -> Result<(), Box<std::error::Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    // let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    // let addr = "127.0.0.1:8080";
    let addr = PORT.parse::<SocketAddr>()?;

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop, so we pass in a handle
    // to our event loop. After the socket's created we inform that we're ready
    // to go and start accepting connections.
    let socket = TcpListener::bind(&addr)?;
    println!("Listening on: {}", addr);

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` method.
    //
    // This combinator, defined on the `Stream` trait, will allow us to define a
    // computation to happen for all items on the stream (in this case TCP
    // connections made to the server).  The return value of the `for_each`
    // method is itself a future representing processing the entire stream of
    // connections, and ends up being our server.
    let done = socket.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Once we're inside this closure this represents an accepted client
            // from our server. The `socket` is the client connection (similar to
            // how the standard library operates).
            //
            // We just want to copy all data read from the socket back onto the
            // socket itself (e.g. "echo"). We can use the standard `io::copy`
            // combinator in the `tokio-core` crate to do precisely this!
            //
            // The `copy` function takes two arguments, where to read from and where
            // to write to. We only have one argument, though, with `socket`.
            // Luckily there's a method, `Io::split`, which will split an Read/Write
            // stream into its two halves. This operation allows us to work with
            // each stream independently, such as pass them as two arguments to the
            // `copy` function.
            //
            // The `copy` function then returns a future, and this future will be
            // resolved when the copying operation is complete, resolving to the
            // amount of data that was copied.
            let (reader, writer) = socket.split();
            let amt = io::copy(reader, writer);

            // After our copy operation is complete we just print out some helpful
            // information.
            let msg = amt.then(move |result| {
                match result {
                    Ok((amt, _, _)) => info!("wrote {} bytes", amt),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });


            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // This function will transfer ownership of the future (`msg` in this
            // case) to the Tokio runtime thread pool that. The thread pool will
            // drive the future to completion.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
            tokio::spawn(msg)
        });

    // And finally now that we've define what our server is, we run it!
    //
    // This starts the Tokio runtime, spawns the server task, and blocks the
    // current thread until all tasks complete execution. Since the `done` task
    // never completes (it just keeps accepting sockets), `tokio::run` blocks
    // forever (until ctrl-c is pressed).
    tokio::run(done);
    Ok(())
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

// #[cfg(not(feature = "yaml"))]
// fn main() {
//     // As stated above, if clap is not compiled with the YAML feature, it is disabled.
//     println!("YAML feature is disabled.");
//     println!("Pass --features yaml to cargo when trying this example.");
// }