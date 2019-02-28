use std::process::Command;

pub fn run(command: &str, silent: bool) {
    let split = command.split(" ").collect::<Vec<&str>>();

    if !silent {
        println!("Running '{}'", command);
    }

    // use the right shell based on operationg system
    let cmd = Command::new("cmd.exe")
        .args(&["/c", command])
        .output()
        .expect("failed to run");

    let stdout = cmd.stdout;
    let stderr = cmd.stderr;
}
