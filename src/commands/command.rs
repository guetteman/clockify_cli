use std::str::FromStr;

pub enum Command {
    Clockify,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "clockify" => Ok(Command::Clockify),
            _ => Err(()),
        }
    }
}
