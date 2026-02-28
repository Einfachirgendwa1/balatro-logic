use balatro_logic::{
    builders::run::RunCreator,
    consumable::{Consumable::TarotCard, Tarot},
    decks::DeckType,
    joker::{Joker, JokerType},
    shop::ShopItem,
    stake::Stake,
    vouchers::Voucher,
};

#[test]
fn vouchers() {
    let mut run = RunCreator::builder()
        .deck(DeckType::Red)
        .stake(Stake::White)
        .seed("AAAAAAAA".to_string())
        .build()
        .create();

    run.data.ante = 0;
    assert_eq!(run.data.poll_next_voucher(), Voucher::TarotMerchant);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::PlanetMerchant);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::MagicTrick);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::SeedMoney);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::Telescope);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::PaintBrush);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::SeedMoney);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::Hieroglyph);
    run.data.ante += 1;
    assert_eq!(run.data.poll_next_voucher(), Voucher::Blank);

    run.data.ante = 1;
    assert!(
        matches!(run.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , .. }) if joker_type == JokerType::Bull)
    );
    assert!(
        matches!(run.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , .. }) if joker_type == JokerType::FacelessJoker)
    );
    run.poll_next_shop_item();
    assert_eq!(run.poll_next_shop_item(), ShopItem::Consumable(TarotCard(Tarot::TheSun)));

    run.data.ante = 2;
    assert!(
        matches!(run.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , .. }) if joker_type == JokerType::Bloodstone)
    );
}
