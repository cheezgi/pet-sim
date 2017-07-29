
#![feature(vec_remove_item)]

mod pet;
mod clamp;
mod cmd;
mod amt;
mod item;
mod messages;

pub use pet::*;
pub use cmd::*;
pub use messages::*;

use std::io;
use std::io::Write;

use amt::Amount;
use clamp::Clamp;

pub fn get_input() -> String {
    io::stdout().flush().expect("Could not flush stdout");

    let mut s = String::new();
    let handle = io::stdin();

    handle.read_line(&mut s).expect("Could not read line");

    s.trim().to_owned()
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

