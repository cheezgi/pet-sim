
use amt::Amount;

pub enum Command {
    Empty,
    Quit,
    Stats,
    Help,
    Xyzzy,
    Feed(Amount),
    Play(Amount),
    Work(Amount),
    Bathe(Amount),
    Give(String),
    Take(String),
}

pub fn parse_command(input: &str) -> Result<Command, ()> {
    let words = input.split_whitespace().collect::<Vec<&str>>();

    if words.is_empty() {
        return Ok(Command::Empty);
    }

    match words[0] {
        "quit" => Ok(Command::Quit),

        "stats" => Ok(Command::Stats),

        "help" => Ok(Command::Help),

        "xyzzy" => Ok(Command::Xyzzy),

        "feed" => {
            if words.len() < 2 {
                Err(())
            } else if words[1] == "a" {
                Ok(Command::Feed(Amount::from(words[2])?))
            } else {
                Ok(Command::Feed(Amount::from(words[1])?))
            }
        },

        "play" => {
            if words.len() < 2 {
                Err(())
            } else if words[1] == "a" {
                Ok(Command::Play(Amount::from(words[2])?))
            } else {
                Ok(Command::Play(Amount::from(words[1])?))
            }
        },

        "work" => {
            if words.len() < 2 {
                Err(())
            } else if words[1] == "a" {
                Ok(Command::Work(Amount::from(words[2])?))
            } else {
                Ok(Command::Work(Amount::from(words[1])?))
            }
        },

        "bathe" | "clean" => {
            if words.len() < 2 {
                Err(())
            } else if words[1] == "a" {
                Ok(Command::Bathe(Amount::from(words[2])?))
            } else {
                Ok(Command::Bathe(Amount::from(words[1])?))
            }
        }

        "give" => {
            if words.len() < 2 {
                Err(())
            } else {
                Ok(Command::Give(words[1..].join(" ")))
            }
        },

        "take" => {
            if words.len() < 2 {
                Err(())
            } else {
                Ok(Command::Take(words[1..].join(" ")))
            }
        }

        _ => Err(())
    }
}

