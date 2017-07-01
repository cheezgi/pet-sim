
pub trait Clamp {
    fn clamp(self, upper: Self, lower: Self) -> Self;
}

impl<T> Clamp for T where T: ::std::cmp::Ord {
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

