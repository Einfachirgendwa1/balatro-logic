use balatro_logic::{
    blind::BossBlindType::{
        TheFish, TheFlint, TheHouse, TheManacle, TheTooth, TheWall, TheWindow, VerdantLeaf,
    },
    builders::run::RunCreator,
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
