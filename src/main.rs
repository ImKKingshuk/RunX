

use std::env;
use std::path::PathBuf;
use runx::{check, run, CONFIGS };


fn main() {
    let mut current = env::current_dir().unwrap();
    let mut args = env::args().skip(1);

  
    if let Some(arg) = args.next() {
        current = PathBuf::from(arg);
    }

    loop {
        if let Some(c) = check(&current) {
           
            println!("Executing {}...", c.0);

            
            if args.any(|arg| arg == "--dry-run") {
                println!("Dry-run mode. Exiting without executing the command.");
                std::process::exit(0);
            }

          
            let tool_args: Vec<String> = args.collect();
            let mut command_args: Vec<&str> = c.1.iter().map(|&s| s).collect(); 
            command_args.extend(tool_args.iter().map(|s| s.as_str())); 
            
            run(&(c.0, &command_args));


            if args.any(|arg| arg == "--continue-search") {
                if let Some(parent) = current.parent() {
                    current = parent.to_owned();
                    continue;
                }
            }

            break;
        }

        match current.parent() {
            Some(c) => current = c.to_owned(),
            None => {
                eprintln!("Unable to find config!");
                eprintln!(
                    "Supported configs:\n{}",
                    CONFIGS
                        .iter()
                        .map(|c| "- ".to_owned() + c.0)
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                std::process::exit(1);
            }
        }
    }
}
