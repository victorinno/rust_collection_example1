use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("'Please use cargo bench' command.");

    let output = Command::new("cargo")
        .arg("bench")
        .output()
        .expect("cargo bench failed to start");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
