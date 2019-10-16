const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

const COMMAND_ARG: &'static str = "command";
const VERBOSE_ARG: &'static str = "verbose";

use clap::{App, Arg};

mod idgaf;

fn main() {
    let matches = App::new(APP_NAME)
        .author(AUTHOR)
        .version(VERSION)
        .arg(
            Arg::with_name(COMMAND_ARG)
                .required(true)
                .help("The command to execute "),
        )
        .arg(
            Arg::with_name(VERBOSE_ARG)
                .short("v")
                .long(VERBOSE_ARG)
                .help("Shows some debug information"),
        )
        .get_matches();

    if matches.is_present(COMMAND_ARG) {
        idgaf::run(
            matches.value_of(COMMAND_ARG).unwrap(),
            matches.is_present(VERBOSE_ARG),
        );
    }
}
