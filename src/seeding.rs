use crate::run::RunData;
use rand::{prelude::*, rng};
use std::{f64::consts::PI, iter::repeat_with};

mod math {
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
            rng.lua_randomseed
                .call::<()>(seed)
                .expect("Call to math.randomseed failed!")
        });
    }

    pub(crate) fn random_idx(len: usize) -> usize {
        RNG.with(|rng| {
            rng.lua_random
                .call::<usize>(len)
                .expect("Call to math.random failed!")
                - 1
        })
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

    for i in (1..list.len()).rev() {
        list.swap(i, math::random_idx(i));
    }
}

pub fn random_element<T>(list: &[T], seed: f64) -> &'_ T {
    math::randomseed(seed);
    &list[math::random_idx(list.len())]
}

impl RunData {
    pub fn seed(&mut self, key: &str) -> f64 {
        let value = self
            .pseudorandom_state
            .entry(key.to_string())
            .or_insert_with_key(|key| hash(format!("{key}{}", self.seed).as_bytes()));

        *value = (((2.134453429141 + *value * 1.72431234) % 1.) * 1e13).round() / 1e13;

        (*value + self.hashed_seed) / 2.0
    }
}
