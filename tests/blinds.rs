use balatro_logic::{
    blind::BossBlindType::{
        TheFish, TheFlint, TheHouse, TheManacle, TheTooth, TheWall, TheWindow, VerdantLeaf,
    },
    builders::run::RunCreator,
    tags::Tag::{
        BossTag, CharmTag, CouponTag, D6Tag, EconomyTag, EtherealTag, GarbageTag, JuggleTag,
        NegativeTag, RareTag, SpeedTag, StandardTag, UncommonTag,
    },
};

#[test]
fn boss_blinds() {
    let mut run = RunCreator::builder().seed("AAAAAAAA".to_string()).build().create();
    assert_eq!(run.data.this_antes_boss, TheManacle);

    let mut next_boss_is = |boss| {
        run.data.ante += 1;
        assert_eq!(run.data.current_boss(), boss);
    };

    next_boss_is(TheHouse);
    next_boss_is(TheFish);
    next_boss_is(TheWindow);
    next_boss_is(TheWall);
    next_boss_is(TheTooth);
    next_boss_is(TheFlint);
    next_boss_is(VerdantLeaf);
}

#[test]
fn skip_tags() {
    let mut run = RunCreator::builder().seed("AAAAAAAA".to_string()).build().create();
    let expected = [
        (SpeedTag, EconomyTag),
        (JuggleTag, EtherealTag),
        (CouponTag, UncommonTag),
        (CouponTag, D6Tag),
        (CouponTag, GarbageTag),
        (StandardTag, RareTag),
        (BossTag, EtherealTag),
        (CharmTag, NegativeTag),
    ];

    for (fst, snd) in expected.iter() {
        assert_eq!(run.data.next_random_tag(), *fst);
        assert_eq!(run.data.next_random_tag(), *snd);
        run.data.ante += 1;
    }
}
