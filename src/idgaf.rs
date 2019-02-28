use std::fs::File;
use std::io::Write;
use std::process::Command;

use sanitize_filename;

pub fn run(command: &str, silent: bool) {
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

    let filename = sanitize_filename::sanitize(command);

    let mut f = File::create(format!("{}.out.log", filename)).unwrap();
    f.write_all(&stdout).unwrap();

    let mut f = File::create(format!("{}.err.log", filename)).unwrap();
    f.write_all(&stderr).unwrap();
}
