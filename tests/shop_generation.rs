use balatro_logic::{decks::DeckType, run::Run, stake::Stake, vouchers::Voucher};

#[test]
fn vouchers() {
    let mut run = Run::new(DeckType::Red, Stake::White, "AAAAAAAA".to_string());
    let mut ante = 0;
    let mut predict_next_voucher = |expected| {
        assert_eq!(run.data.poll_next_voucher(ante), expected);
        ante += 1;
    };

    predict_next_voucher(Voucher::TarotMerchant);
    predict_next_voucher(Voucher::PlanetMerchant);
    predict_next_voucher(Voucher::MagicTrick);
    predict_next_voucher(Voucher::SeedMoney);
    predict_next_voucher(Voucher::Telescope);
    predict_next_voucher(Voucher::PaintBrush);
    predict_next_voucher(Voucher::SeedMoney);
    predict_next_voucher(Voucher::Hieroglyph);
    predict_next_voucher(Voucher::Blank);
}
