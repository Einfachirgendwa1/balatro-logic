use crate::{run::RunData, seeding::random_idx, vouchers::Voucher};
use num_traits::FromPrimitive;
use strum::{EnumCount, IntoEnumIterator};

impl RunData {
    pub fn available_vouchers(&self) -> [bool; Voucher::COUNT] {
        let mut v = [false; Voucher::COUNT];

        for voucher in Voucher::iter() {
            let allowed = !self.vouchers[voucher as usize]
                && voucher.requirements_fulfilled(&self.vouchers)
                && !self.shop.vouchers.contains(&voucher);

            v[voucher as usize] = allowed;
        }

        if !v.iter().any(|available| *available) {
            v[Voucher::Blank as usize] = true;
        }

        v
    }

    pub fn poll_next_voucher(&mut self, ante: u32) -> Voucher {
        let available = self.available_vouchers();

        let seed = self.rng.seed(&format!("Voucher{ante}"));

        let mut idx = random_idx(&available, seed);
        let mut i = 1;

        while !available[idx] {
            i += 1;

            let seed = self.rng.seed(&format!("Voucher{ante}_resample{i}"));
            idx = random_idx(&available, seed);
        }

        Voucher::from_usize(idx).unwrap()
    }
}
