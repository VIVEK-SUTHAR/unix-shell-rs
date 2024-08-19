use std::io::{self, Write};
mod command;
mod supported;
use crate::command::process::process_command;
const SHELL_CHARACTER: &str = "$";
mod commands;
fn main() {
    loop {
        print_promt();

        let command_input = read_command();

        let tokenized_input = command::tokenize::tokenize_command(command_input);

        process_command(tokenized_input);
    }
}

fn print_promt() {
    print!("{0} ", SHELL_CHARACTER);
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut command_input = String::new();
    io::stdin()
        .read_line(&mut command_input)
        .expect("Failed to read line");
    command_input
}
