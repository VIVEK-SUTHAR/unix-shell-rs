use crate::command;
use crate::supported::commands::SupportedCommands;
use crate::commands::{ls,echo,cat};
use std::env;
use std::process;
use std::str::FromStr;
pub fn process_command(command_input: command::tokenize::CommandInput) -> i32 {
    match SupportedCommands::from_str(&command_input.command) {
        Ok(SupportedCommands::Echo) => echo::my_echo(&command_input.argument),
        Ok(SupportedCommands::Clear) => clear(),
        Ok(SupportedCommands::Pwd) => get_current_working_dir(),
        Ok(SupportedCommands::Exit) => exit_from_current_process(),
        Ok(SupportedCommands::Ls) => ls::print_contents_of_current_directory(),
        Ok(SupportedCommands::Cat)=>cat::process_cat(&command_input.argument),
        _ => {
            println!("{}: command not found", &command_input.command);
            1
        }
    }
}

fn clear() -> i32 {
    //Escape sequences
    print!("\x1B[2J\x1B[1;1H");
    0
}

fn get_current_working_dir() -> i32 {
    let path = env::current_dir();
    match path {
        Ok(path_buf) => {
            println!("{}", path_buf.display());
            0
        }
        Err(_) => {
            println!("Failed to get the current working directory");
            -1
        }
    }
}

fn exit_from_current_process() -> i32 {
    print!("\x1B[2J\x1B[1;1H");
    process::exit(0);
}
