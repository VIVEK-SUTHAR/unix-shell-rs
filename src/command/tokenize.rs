pub struct CommandInput {
    pub command: String,
    pub argument: Vec<String>,
}

pub fn tokenize_command(input_command: String) -> CommandInput {
    let mut command_split: Vec<String> = input_command
        .split_whitespace()
        .map(|str| str.to_string())
        .collect();
    let command = CommandInput {
        command: command_split.remove(0),
        argument: command_split,
    };
    command
}
