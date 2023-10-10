const COMMAND_ARG: &'static str = "command";
const VERBOSE_ARG: &'static str = "verbose";

use clap::{command, Arg, ArgAction};

mod idgaf;

fn main() {
    let matches = command!()
        .arg(
            Arg::new(COMMAND_ARG)
                .required(true)
                .help("The command to execute "),
        )
        .arg(
            Arg::new(VERBOSE_ARG)
                .short('v')
                .long(VERBOSE_ARG)
                .action(ArgAction::SetTrue)
                .help("Shows some debug information"),
        )
        .get_matches();

    if matches.contains_id(COMMAND_ARG) {
        idgaf::run(
            matches.get_one::<String>(COMMAND_ARG).unwrap(),
            matches.get_flag(VERBOSE_ARG),
        );
    }
}
