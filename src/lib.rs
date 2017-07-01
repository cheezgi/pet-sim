
#![feature(vec_remove_item)]

mod pet;
pub use pet::*;

use std::io;
use std::io::Write;

pub fn get_input() -> String {
    io::stdout().flush().expect("Could not flush stdout");

    let mut s = String::new();
    let handle = io::stdin();

    handle.read_line(&mut s).expect("Could not read line");

    s.trim().to_owned()
}


pub fn parse_command(input: &str) -> Result<Command, ()> {
    match input {
        "quit" | "exit" => Ok(Command::Quit),
        _ => Err(())
    }
}

pub enum Command {
    Quit,
}

trait Clamp {
    fn clamp(self, upper: Self, lower: Self) -> Self;
}

impl<T> Clamp for T where T: std::cmp::Ord {
    fn clamp(self, lower: T, upper: T) -> T {
        if self > upper {
            upper
        } else if self < lower {
            lower
        } else {
            self
        }
    }
}

#[cfg(test)]
mod test {
    use Clamp;

    #[test]
    fn clamp_between_returns_self() {
        let i = 50;
        assert_eq!(50, i.clamp(0, 100));
    }

    #[test]
    fn clamp_above_returns_upper() {
        let i = 150;
        assert_eq!(100, i.clamp(0, 100));
    }

    #[test]
    fn clamp_below_returns_lower() {
        let i = -150;
        assert_eq!(0, i.clamp(0, 100));
    }
}

