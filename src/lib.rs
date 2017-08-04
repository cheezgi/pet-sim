
#![feature(vec_remove_item)]

extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod pet;
mod clamp;
mod cmd;
mod amt;
mod item;
mod messages;
mod player;
mod save;

pub use pet::*;
pub use cmd::*;
pub use messages::*;
pub use player::*;
pub use save::*;

use amt::Amount;
use clamp::Clamp;

use std::io;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::path::Path;


pub fn get_input() -> String {
    io::stdout().flush().expect("Could not flush stdout");

    let mut s = String::new();
    let handle = io::stdin();

    handle.read_line(&mut s).expect("Could not read line");

    s.trim().to_owned()
}

pub fn read_settings() -> Settings {
    serde_json::from_str(
        &if let Ok(mut file) = File::open(Path::new("settings.json")) {
            let mut s = String::new();
            file.read_to_string(&mut s).expect("Could not read file");
            s
        } else {
            panic!("Could not open file")
        }
    ).expect("Settings malformed")
}

pub fn add_scale(lhs: u8, rhs: Amount, scale: u8) -> u8 {
    (lhs as i16 + rhs.scale(scale) as i16).clamp(0, 100) as u8
}

pub fn sub_scale(lhs: u8, rhs: Amount, scale: u8) -> u8 {
    (lhs as i16 - rhs.scale(scale) as i16).clamp(0, 100) as u8
}

#[cfg(test)]
mod test {
    use ::clamp::Clamp;

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

