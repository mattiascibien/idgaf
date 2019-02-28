const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

const COMMAND_ARG: &'static str = "command";

use clap::{App, Arg};

mod idgaf;

fn main() {
    let matches = App::new(APP_NAME)
        .author(AUTHOR)
        .version(VERSION)
        .arg(Arg::with_name(COMMAND_ARG).required(true))
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();

    if matches.is_present("command") {
        idgaf::run(
            matches.value_of("command").unwrap(),
            matches.is_present("silent"),
        );
    }
}
