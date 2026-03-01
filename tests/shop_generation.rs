use balatro_logic::{
    boosters::{
        BoosterPackData,
        BoosterPackType::{BuffoonNormal, CelestialJumbo},
    },
    builders::run::RunCreator,
    consumable::{Consumable::TarotCard, Tarot},
    hands::HandType::{FourOfAKind, HighCard, Pair, ThreeOfAKind, TwoPair},
    joker::{Joker, JokerType},
    shop::ShopItem,
    vouchers::Voucher,
};

#[test]
fn vouchers() {
    let mut run = RunCreator::builder().seed("AAAAAAAA".to_string()).build().create();
    run.data.ante = 0;

    let mut next_voucher = |voucher| {
        run.enter_shop(true);
        run.data.ante += 1;

        assert_eq!(run.data.shop.vouchers[0], voucher);
    };

    next_voucher(Voucher::TarotMerchant);
    next_voucher(Voucher::PlanetMerchant);
    next_voucher(Voucher::MagicTrick);
    next_voucher(Voucher::SeedMoney);
    next_voucher(Voucher::Telescope);
    next_voucher(Voucher::PaintBrush);
    next_voucher(Voucher::SeedMoney);
    next_voucher(Voucher::Hieroglyph);
    next_voucher(Voucher::Blank);
}

#[test]
fn shop_inventory() {
    fn is(shop_item: &ShopItem, j_type: JokerType) -> bool {
        matches!(shop_item, ShopItem::Joker(Joker { joker_type , .. }) if joker_type == &j_type)
    }
    let mut run = RunCreator::builder().seed("AAAAAAAA".to_string()).build().create();
    run.enter_shop(true);

    assert!(is(&run.data.shop.inventory[0], JokerType::Bull));
    assert!(is(&run.data.shop.inventory[1], JokerType::FacelessJoker));

    run.reroll();
    assert!(is(&run.data.shop.inventory[0], JokerType::Misprint));
    assert_eq!(&run.data.shop.inventory[1], &ShopItem::Consumable(TarotCard(Tarot::TheSun)));

    run.data.ante = 2;
    run.enter_shop(true);

    assert!(is(&run.data.shop.inventory[0], JokerType::Bloodstone));
}

#[test]
fn shop_packs() {
    fn is(joker: &Joker, j_type: JokerType) -> bool {
        matches!(joker, Joker { joker_type , .. } if joker_type == &j_type)
    }

    let mut run = RunCreator::builder().seed("AAAAAAAA".to_string()).build().create();
    run.enter_shop(true);

    assert_eq!(run.data.shop.packs, [Some(BuffoonNormal), Some(CelestialJumbo)]);

    match run.open_booster_pack(BuffoonNormal) {
        BoosterPackData::BuffoonNormal([a, b]) => {
            assert!(is(&a, JokerType::ShootTheMoon));
            assert!(is(&b, JokerType::ZanyJoker));
        }
        _ => unreachable!(),
    }

    let e = [ThreeOfAKind.into(), Pair.into(), FourOfAKind.into(), TwoPair.into(), HighCard.into()];
    match run.open_booster_pack(CelestialJumbo) {
        BoosterPackData::CelestialJumbo(list) => {
            assert_eq!(list, e);
        }
        _ => unreachable!(),
    }
}
