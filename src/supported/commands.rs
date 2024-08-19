use std::str::FromStr;
pub enum SupportedCommands {
    Echo,
    History,
    Clear,
    Cd,
    Pwd,
    Exit,
    Ls,
    Cat
}
impl FromStr for SupportedCommands {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(SupportedCommands::Echo),
            "history" => Ok(SupportedCommands::History),
            "clear" => Ok(SupportedCommands::Clear),
            "cd" => Ok(SupportedCommands::Cd),
            "pwd" => Ok(SupportedCommands::Pwd),
            "ls" => Ok(SupportedCommands::Ls),
            "exit" => Ok(SupportedCommands::Exit),
            "cat"=>Ok(SupportedCommands::Cat),
            _ => Err(()),
        }
    }
}
