use crate::misc::Also;
use rand::{prelude::*, rng};
use std::{collections::HashMap, f64::consts::PI, iter::repeat_with};

pub(crate) mod math {
    use mlua::{Function, Lua, Table};
    use std::cell::LazyCell;

    struct LuaRng {
        lua_randomseed: Function,
        lua_random: Function,
    }

    thread_local! {
        pub static LUA: LazyCell<Lua> = LazyCell::new(Lua::new);
        static RNG: LazyCell<LuaRng> = LazyCell::new(|| {
            let math: Table = LUA.with(|v| v.globals().get("math").unwrap());

            LuaRng {
                lua_random: math.get("random").expect("math.random not found!"),
                lua_randomseed: math.get("randomseed").expect("math.randomseed not found!"),
            }
        });
    }

    pub(crate) fn randomseed(seed: f64) {
        RNG.with(|rng| {
            rng.lua_randomseed.call::<()>(seed).expect("Call to math.randomseed failed!")
        });
    }

    pub(crate) fn random() -> f64 {
        RNG.with(|rng| rng.lua_random.call::<f64>(()).expect("Call to math.random failed!"))
    }

    pub(crate) fn random_idx(len: usize) -> usize {
        RNG.with(|rng| rng.lua_random.call::<usize>(len).expect("Call to math.random failed!") - 1)
    }
}

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

#[inline]
pub const fn hash(s: &[u8]) -> f64 {
    let mut num = 1.;
    let mut i = s.len();

    while i != 0 {
        let byte = s[i - 1];

        let magic_division = 1.1239285023 / num;
        num = ((magic_division * byte as f64 * PI) + (PI * i as f64)) % 1.;

        i -= 1;
    }
    num
}

/// O(n) Fisher-Yates
pub fn shuffle<T>(list: &mut [T], seed: f64) {
    math::randomseed(seed);

    for i in (1..=list.len()).rev() {
        let j = math::random_idx(i);
        list.swap(i - 1, j);
    }
}

pub fn random_element<T>(list: &[T], seed: f64) -> &'_ T {
    &list[random_idx(list, seed)]
}

pub fn random_idx<T>(list: &[T], seed: f64) -> usize {
    math::randomseed(seed);
    math::random_idx(list.len())
}

pub struct BalatroRng {
    pub seed: String,
    pub hashed_seed: f64,
    pub pseudorandom_state: HashMap<String, f64>,
}

impl BalatroRng {
    #[inline]
    #[must_use]
    pub fn new(seed: String) -> BalatroRng {
        BalatroRng { hashed_seed: hash(seed.as_bytes()), seed, pseudorandom_state: HashMap::new() }
    }

    #[inline]
    #[must_use]
    pub fn new_randomseed() -> BalatroRng {
        BalatroRng::new(random_seed())
    }

    #[inline]
    #[must_use]
    pub fn seed_one(&self, key: &str) -> f64 {
        let hashed = hash(format!("{key}{}", self.seed).as_bytes());
        let value = (((2.134453429141 + hashed * 1.72431234) % 1.) * 1e13).round() / 1e13;

        (value + self.hashed_seed) / 2.0
    }

    pub fn seed(&mut self, key: &str) -> f64 {
        let value = self
            .pseudorandom_state
            .entry(key.to_string())
            .or_insert_with_key(|key| hash(format!("{key}{}", self.seed).as_bytes()));

        *value = (((2.134453429141 + *value * 1.72431234) % 1.) * 1e13).round() / 1e13;
        ((*value + self.hashed_seed) / 2.0).also(|val| println!("Seeding on `{key}` yields {val}"))
    }
}
