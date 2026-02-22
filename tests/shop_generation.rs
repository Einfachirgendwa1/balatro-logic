use balatro_logic::{
    decks::DeckType,
    joker::{Joker, JokerType},
    pools::ShopItem,
    run::Run,
    stake::Stake,
    vouchers::Voucher,
};

#[test]
fn vouchers() {
    let mut run = Run::new(DeckType::Red, Stake::White, "AAAAAAAA".to_string());

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
        matches!(run.data.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , ..}) if joker_type == JokerType::Bull)
    );
    assert!(
        matches!(run.data.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , ..}) if joker_type == JokerType::FacelessJoker)
    );
    run.data.ante = 2;
    assert!(
        matches!(run.data.poll_next_shop_item(), ShopItem::Joker(Joker { joker_type , ..}) if joker_type == JokerType::Bloodstone)
    );
}
