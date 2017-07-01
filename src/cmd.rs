
pub fn parse_command(input: &str) -> Result<Command, ()> {
    match input {
        "quit" | "exit" => Ok(Command::Quit),
        _ => Err(())
    }
}

pub enum Command {
    Quit,
}

