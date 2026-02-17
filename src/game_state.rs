use crate::blind::Blind;

pub enum GameState {
    Blind(Blind),
    Shop,
}
