use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        let mut buffer = String::new();
        print!("$ ");
        io::stdout().flush().unwrap(); // for print prompt before input
        io::stdin().read_line(&mut buffer).unwrap(); // input command

        let commands: Vec<&str> = buffer.trim_end().split(" ").collect();

        match commands[0] {
            "cd" => {
                println!("path: {}", commands[1]); // ignore after seconds argument
                let path = Path::new(commands[1]);
                let ret = env::set_current_dir(path);
                match ret {
                    Ok(_) => println!("change success"),
                    Err(_) => println!("Directory \"{}\" is not exists", path.to_str().unwrap()),
                }
                continue;
            }
            _ => (),
        }

        let output = Command::new(commands[0])
            .args(&commands[1..])
            .output()
            .expect("failed to execute command");
        let output_str = String::from_utf8_lossy(&output.stdout);

        print!("{}", output_str);
    }
}
