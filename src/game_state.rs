use crate::blind::Blind;

#[derive(Debug)]
pub enum GameState {
    Shop,
    BlindSelection,
    Blind(Blind),
    CashOut,
}
