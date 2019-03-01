use std::fs::File;
use std::io::Write;
use std::process::Command;

use sanitize_filename;

pub fn run(command: &str, silent: bool) {
    if let Ok(ps) = pentry::current() {
        let parent_executable = ps.parent().unwrap();

        if !silent {
            println!("Running '{}' using {}", command, parent_executable.name());
        }

        // use the right shell based on operationg system
        let cmd = Command::new(parent_executable.name())
            .args(&[get_run_command_parameter(parent_executable.name()), command])
            .output()
            .expect("failed to run");

        let stdout = cmd.stdout;
        let stderr = cmd.stderr;

        let filename = sanitize_filename::sanitize(command);

        let mut f = File::create(format!("{}.out.log", filename)).unwrap();
        f.write_all(&stdout).unwrap();

        let mut f = File::create(format!("{}.err.log", filename)).unwrap();
        f.write_all(&stderr).unwrap();
    };
}

fn get_run_command_parameter(shell: &str) -> &'static str {
    if shell.starts_with("powershell") || shell.starts_with("pwsh") {
        "-command"
    } else if shell.starts_with("cmd") {
        "/c"
    } else if shell.starts_with("bash") || shell.starts_with("sh") {
        "-c"
    } else {
        panic!("shell {} not supported", shell);
    }
}
