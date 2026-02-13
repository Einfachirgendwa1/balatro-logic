extern crate core;

pub mod blind;
mod consumable;
mod decks;
pub mod event;
pub mod hands;
pub mod joker;
pub mod run;
mod seeding;
pub mod stake;
mod vouchers;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
