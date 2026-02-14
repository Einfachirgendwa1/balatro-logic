#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Stake {
    White,
    Red,
    Green,
    Black,
    Blue,
    Purple,
    Orange,
    Gold,
}
