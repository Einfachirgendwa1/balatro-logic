use crate::run::Run;
use consts::PI;
use rand::{prelude::*, rng};
use std::{f64::consts, iter::repeat_with};

pub fn random_seed() -> String {
    let mut rng = rng();

    repeat_with(|| {
        if rng.random_bool(0.3) {
            rng.random_range('1'..='9')
        } else if rng.random_bool(0.55) {
            rng.random_range('A'..='N')
        } else {
            rng.random_range('P'..='Z')
        }
    })
    .take(8)
    .collect()
}

pub fn hash(s: &str) -> f64 {
    let mut num = 1.0_f64;

    for (i, byte) in s.bytes().rev().enumerate() {
        let idx = s.len() - i;
        num = (PI * ((1.1239285023 / num) * byte as f64 + idx as f64)) % 1.0;
    }

    num
}

/// O(n) Fisher-Yates
pub fn shuffle<T>(list: &mut [T], seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    for i in (1..list.len()).rev() {
        let j = rng.random_range(0..=i);
        list.swap(i, j);
    }
}

impl Run {
    pub(crate) fn seed_channel(&mut self, channel: String) -> f64 {
        let chan_value = self
            .pseudorandom_state
            .entry(channel)
            .or_insert_with_key(|key| hash(&format!("{key}{}", self.seed)));

        *chan_value = (2.134453429141 + *chan_value * 1.72431234) % 1.0;

        (*chan_value + self.hashed_seed) / 2.0
    }
}
