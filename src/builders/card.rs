use crate::{
    card::{Card, Edition, Enhancement, Seal},
    decks::DEFAULT_CARDS,
    run::RunData,
    seeding::random_element,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct CardCreator<'a> {
    origin_key: &'a str,

    #[builder(default=Enhancement::None)]
    enhancement: Enhancement,

    #[builder(default=Edition::Base)]
    edition: Edition,

    #[builder(default=Seal::None)]
    seal: Seal,
}

impl CardCreator<'_> {
    pub fn create(self, data: &mut RunData) -> Card {
        let Self { origin_key, edition, enhancement, seal } = self;
        let seed = data.rng.seed(&format!("front{origin_key}{}", data.ante));

        let mut card = random_element(&DEFAULT_CARDS, seed).clone();

        card.enhancement = enhancement;
        card.edition = edition;
        card.seal = seal;

        card
    }
}
