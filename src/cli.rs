use clap::{App};

pub fn build_cli<'a>() -> App<'static, 'static> {
    App::from_yaml(load_yaml!("17_yaml.yml"))
}