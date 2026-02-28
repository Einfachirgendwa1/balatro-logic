use crate::{
    builders::{
        card::CardCreator,
        consumable::ConsumableCreator,
        joker::{JokerCreator, JokerRarityMode},
    },
    card::Card,
    consumable::{PLANET_ORDER, Planet, Spectral, Spectral::BlackHole, Tarot},
    hands::HandType,
    joker::Joker,
    run::{Run, RunData},
    seeding::math,
    vouchers::Voucher,
};
use Spectral::TheSoul;
use derive_more::From;
use itertools::Itertools;
use num_derive::FromPrimitive;
use std::array;
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive, Display)]
pub enum BoosterPackType {
    ArcanaNormal,
    ArcanaJumbo,
    ArcanaMega,
    CelestialNormal,
    CelestialJumbo,
    CelestialMega,
    StandardNormal,
    StandardJumbo,
    StandardMega,
    BuffoonNormal,
    BuffoonJumbo,
    BuffoonMega,
    SpectralNormal,
    SpectralJumbo,
    SpectralMega,
}

#[derive(Debug, Clone, PartialEq, EnumCount)]
pub enum BoosterPackData {
    ArcanaNormal([ArcanaCard; 3]),
    ArcanaJumbo([ArcanaCard; 5]),
    ArcanaMega([ArcanaCard; 5]),
    CelestialNormal([CelestialCard; 3]),
    CelestialJumbo([CelestialCard; 5]),
    CelestialMega([CelestialCard; 5]),
    StandardNormal([Card; 3]),
    StandardJumbo([Card; 5]),
    StandardMega([Card; 5]),
    BuffoonNormal([Joker; 2]),
    BuffoonJumbo([Joker; 4]),
    BuffoonMega([Joker; 4]),
    SpectralNormal([Spectral; 2]),
    SpectralJumbo([Spectral; 4]),
    SpectralMega([Spectral; 4]),
}

#[derive(Debug, Copy, Clone, PartialEq, EnumCount, From)]
pub enum ArcanaCard {
    Tarot(Tarot),
    Spectral(Spectral),
}

#[derive(Debug, Copy, Clone, PartialEq, EnumCount, From)]
pub enum CelestialCard {
    Planet(HandType),
    BlackHole,
}

impl From<Spectral> for CelestialCard {
    fn from(spectral: Spectral) -> Self {
        assert_eq!(spectral, BlackHole);
        Self::BlackHole
    }
}

impl Run {
    pub fn next_shop_booster_pack(&mut self) -> BoosterPackType {
        let Self { data, .. } = self;
        if !data.shop.first_shop_buffoon {
            data.shop.first_shop_buffoon = true;
            return BoosterPackType::BuffoonNormal;
        }

        let sum_of_weight: f64 = data.shop.pack_weights.iter().sum();
        math::randomseed(data.rng.seed(&format!("shop_pack{}", data.ante)));

        let pick = math::random() * sum_of_weight;
        let mut cur_weight = 0.;

        for (pack, weight) in BoosterPackType::iter().zip_eq(data.shop.pack_weights) {
            if pick >= cur_weight && pick <= cur_weight + weight {
                return pack;
            }

            cur_weight += weight;
        }

        unreachable!()
    }

    pub fn open_booster_pack(&mut self, booster_pack_type: BoosterPackType) -> BoosterPackData {
        let Self { data, .. } = self;

        match booster_pack_type {
            BoosterPackType::ArcanaNormal => BoosterPackData::ArcanaNormal(data.arcana()),
            BoosterPackType::ArcanaJumbo => BoosterPackData::ArcanaJumbo(data.arcana()),
            BoosterPackType::ArcanaMega => BoosterPackData::ArcanaMega(data.arcana()),
            BoosterPackType::CelestialNormal => BoosterPackData::CelestialNormal(data.celestial()),
            BoosterPackType::CelestialJumbo => BoosterPackData::CelestialJumbo(data.celestial()),
            BoosterPackType::CelestialMega => BoosterPackData::CelestialMega(data.celestial()),
            BoosterPackType::SpectralNormal => BoosterPackData::SpectralNormal(data.spectral()),
            BoosterPackType::SpectralJumbo => BoosterPackData::SpectralJumbo(data.spectral()),
            BoosterPackType::SpectralMega => BoosterPackData::SpectralMega(data.spectral()),
            BoosterPackType::BuffoonNormal => BoosterPackData::BuffoonNormal(self.jokers()),
            BoosterPackType::BuffoonJumbo => BoosterPackData::BuffoonJumbo(self.jokers()),
            BoosterPackType::BuffoonMega => BoosterPackData::BuffoonMega(self.jokers()),
            BoosterPackType::StandardNormal => BoosterPackData::StandardNormal(data.normal()),
            BoosterPackType::StandardJumbo => BoosterPackData::StandardJumbo(data.normal()),
            BoosterPackType::StandardMega => BoosterPackData::StandardMega(data.normal()),
        }
    }

    fn jokers<const N: usize>(&mut self) -> [Joker; N] {
        let mut jokers = [const { Option::<Joker>::None }; N];

        for idx in 0..N {
            let joker = JokerCreator::builder()
                .origin_key("buf")
                .joker_rarity(JokerRarityMode::RandomNonLegendary)
                .filter(&|joker_type| {
                    !jokers[..idx].iter().any(|j| j.as_ref().unwrap().joker_type == *joker_type)
                })
                .dont_filter_on_showman()
                .build()
                .create(self);

            jokers[idx] = Some(joker);
        }

        jokers.map(Option::unwrap)
    }
}
impl RunData {
    fn arcana<const N: usize>(&mut self) -> [ArcanaCard; N] {
        let mut arcana_cards = [None; N];

        for idx in 0..N {
            let card = if self.vouchers[Voucher::OmenGlobe as usize] && {
                math::randomseed(self.rng.seed("omen_globe"));
                math::random() > 0.8
            } {
                ConsumableCreator::<{ Spectral::COUNT }, Spectral, ArcanaCard>::builder()
                    .type_key("Spectral")
                    .origin_key("ar2")
                    .soul(TheSoul)
                    .filter(&|spectral| {
                        !arcana_cards[..idx].contains(&Some(ArcanaCard::Spectral(*spectral)))
                    })
                    .dont_filter_on_showman()
                    .build()
                    .create(self)
            } else {
                ConsumableCreator::<{ Tarot::COUNT }, Tarot, ArcanaCard>::builder()
                    .type_key("Tarot")
                    .origin_key("ar1")
                    .soul(TheSoul)
                    .filter(&|tarot| {
                        !arcana_cards[..idx].contains(&Some(ArcanaCard::Tarot(*tarot)))
                    })
                    .dont_filter_on_showman()
                    .build()
                    .create(self)
            };

            arcana_cards[idx] = Some(card);
        }

        arcana_cards.map(Option::unwrap)
    }

    fn celestial<const N: usize>(&mut self) -> [CelestialCard; N] {
        let mut celestial_cards = [None; N];
        let unlocked = self.planet_unlocked;

        for idx in 0..N {
            let card = if self.vouchers[Voucher::Telescope as usize] && idx == 0 {
                let highest_hand = HandType::iter()
                    .rev()
                    .max_by_key(|hand| self.times_played[*hand as usize])
                    .unwrap();

                CelestialCard::Planet(highest_hand)
            } else {
                ConsumableCreator::<{ Planet::COUNT }, Planet, CelestialCard>::builder()
                    .type_key("Planet")
                    .origin_key("pl1")
                    .order(PLANET_ORDER)
                    .soul(BlackHole)
                    .filter(&|planet| {
                        !celestial_cards[..idx].contains(&Some(CelestialCard::Planet(*planet)))
                            && unlocked[*planet as usize]
                    })
                    .dont_filter_on_showman()
                    .build()
                    .create(self)
            };

            celestial_cards[idx] = Some(card);
        }

        celestial_cards.map(Option::unwrap)
    }

    fn spectral<const N: usize>(&mut self) -> [Spectral; N] {
        let mut spectral_cards = [None; N];

        for idx in 0..N {
            let card = ConsumableCreator::<{ Spectral::COUNT }, Spectral, Spectral>::builder()
                .type_key("Spectral")
                .origin_key("spe")
                .soul(TheSoul)
                .filter(&|spectral| !spectral_cards[..idx].contains(&Some(*spectral)))
                .dont_filter_on_showman()
                .build()
                .create(self);

            spectral_cards[idx] = Some(card);
        }

        spectral_cards.map(Option::unwrap)
    }

    fn normal<const N: usize>(&mut self) -> [Card; N] {
        array::from_fn(|_| CardCreator::builder().origin_key("sta").build().create(self))
    }
}
