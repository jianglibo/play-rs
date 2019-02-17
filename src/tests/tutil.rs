use std::path::{PathBuf};
use std::env;

pub fn init_log() {
    ::std::env::set_var("RUST_LOG", "play_rs=debug");
    env_logger::init();
}

fn get_final_file(pn: &str, postfix: &[&str], canonicalize: bool) -> std::io::Result<PathBuf> {
    let mut path_result = env::current_dir()?;
    // postfix.insert(0, "abc");
    // path_result = path_result.join(pn);
    let mut v = vec![pn];
    v.extend_from_slice(postfix);
    path_result = v.iter().fold(path_result, |state, x| {
        info!("{}", x);
        state.join(x)
    });
    if canonicalize {
        Ok(path_result.canonicalize()?)
    } else {
        Ok(path_result)
    }
}

#[allow(dead_code)]
pub fn get_fixture_file(postfix: &[&str], canonicalize: bool) -> std::io::Result<PathBuf> {
    get_final_file("fixtures", postfix, canonicalize)
}

pub fn get_out_file(postfix: &[&str]) -> std::io::Result<PathBuf> {
    get_final_file("notingit", postfix, false)
}