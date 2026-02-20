use balatro_logic::{
    blind::Blind,
    card::{
        Card,
        Rank::{Queen, Rank2, Rank7, Rank8, Rank10},
        Suit::{Club, Diamond, Heart, Spade},
    },
    controller::{BlindAction, Controller, SimulationResult},
    decks::DeckType,
    run::RunData,
    seeding::BalatroRng,
    stake::Stake,
};
use itertools::Itertools;

#[test]
fn erratic_bugged_seeds() {
    assert_eq!(
        DeckType::gen_erratic(&mut BalatroRng::new("BGY5SDS".to_string())),
        vec![Card::new(Spade, Rank10); 52]
    )
}

#[test]
fn initial_draw() {
    struct Simulation;

    impl Controller for Simulation {
        fn blind(&mut self, blind: &mut Blind, data: &mut RunData) -> Vec<BlindAction> {
            assert_eq!(
                data.resolve(&blind.held).iter().sorted().collect_vec(),
                [
                    &Card::new(Diamond, Queen),
                    &Card::new(Club, Queen),
                    &Card::new(Heart, Rank10),
                    &Card::new(Spade, Rank8),
                    &Card::new(Diamond, Rank7),
                    &Card::new(Diamond, Rank2),
                    &Card::new(Heart, Rank2),
                    &Card::new(Spade, Rank2)
                ]
                .iter()
                .sorted()
                .collect_vec()
            );

            vec![BlindAction::Abort]
        }
    }

    let res = DeckType::Red
        .new_run("AAAAAAAA".to_string(), Stake::White)
        .simulate(Simulation);

    assert_eq!(res, SimulationResult::Aborted);
}
