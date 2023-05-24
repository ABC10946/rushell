use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut buffer = String::new();
    print!("$ ");
    io::stdout().flush().unwrap(); // for print prompt before input
    io::stdin().read_line(&mut buffer).unwrap(); // input command

    println!("{}", buffer.trim_end());
    let output = Command::new(buffer.trim_end()).output().expect("failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);

    println!("{}", output_str);
}
