
extern crate rand;

use self::rand::Rng;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Amount {
    Lots,
    Some,
    Bit,
    Little,
    None,
}

impl<'a> Amount {
    pub fn from(s: &'a str) -> Result<Amount, ()> {
        match s {
            "lot" | "lots" => Ok(Amount::Lots),
            "some" => Ok(Amount::Some),
            "bit" => Ok(Amount::Bit),
            "little" => Ok(Amount::Little),
            "none" => Ok(Amount::None),
            _ => Err(())
        }
    }

    pub fn scale(self, scale: u8) -> u8 {
        let n: u8 = self.into();
        n / scale
    }
}

impl From<u8> for Amount {
    fn from(n: u8) -> Amount {
        match n {
            0 => Amount::None,
            1...24 => Amount::Little,
            25...49 => Amount::Bit,
            50...75 => Amount::Some,
            _ => Amount::Lots,
        }
    }
}

impl Into<u8> for Amount {
    fn into(self) -> u8 {
        match self {
            Amount::None => 0,
            Amount::Little => 1 + rand::thread_rng().gen_range(0, 26),
            Amount::Bit => 25 + rand::thread_rng().gen_range(0, 26),
            Amount::Some => 50 + rand::thread_rng().gen_range(0, 26),
            Amount::Lots => 75 + rand::thread_rng().gen_range(0, 26),
        }
    }
}

