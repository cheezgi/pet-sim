
use amt::Amount;
use item::Item;

pub enum Command {
    Empty,
    Quit,
    Stats,
    Help,
    Feed(Amount),
    Play(Amount),
    Work(Amount),
    Give(Item),
    Take(Item),
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

        "give" => {
            Err(())
        },

        "take" => {
            Err(())
        }

        _ => Err(())
    }
}

