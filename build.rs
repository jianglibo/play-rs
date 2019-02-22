#[macro_use]
extern crate clap;

use clap::Shell;

// https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html

// include!("src/cli.rs");

fn main() {
    use std::env;
    // let mut app = build_cli();
    let yml = load_yaml!("17_yaml.yml");
    let mut app = App::from_yaml(yml);

    let out_dir = env::var("OUT_DIR").unwrap();
    app.gen_completions(
        "play-rs",           // We specify the bin name manually
        Shell::PowerShell,      // Which shell to build completions for
        out_dir); // Where write the completions to
}