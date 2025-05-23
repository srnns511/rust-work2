use std::io::{self, Write};
use std::process::Command;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    loop{
        print!("输入:");
        io::stdout().flush()?;

        //read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" {
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let output = Command::new(command)
            .args(args)
            .spawn();

        match output {
            Ok(mut child) => {
                child.wait()?;
            }
            Err(e) => {
                eprintln!("执行命令出错: {}", e);
            }
        }
    }

    Ok(())
}
