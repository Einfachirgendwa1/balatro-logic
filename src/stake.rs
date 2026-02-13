#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Stake {
    White,
    Red,
    Green,
    Black,
    Blue,
    Purple,
    Orange,
    Gold,
}
