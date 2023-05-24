use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        let mut buffer = String::new();
        print!("$ ");
        io::stdout().flush().unwrap(); // for print prompt before input
        io::stdin().read_line(&mut buffer).unwrap(); // input command

        let commands: Vec<&str> = buffer.trim_end().split(" ").collect();

        let output = Command::new(commands[0])
            .args(&commands[1..])
            .output()
            .expect("failed to execute command");
        let output_str = String::from_utf8_lossy(&output.stdout);

        print!("{}", output_str);
    }
}
