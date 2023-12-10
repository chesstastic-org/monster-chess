#[test]
fn ataxx_perft_suite() {
    use crate::board::tests::run_tests;
    use super::Ataxx;
    
    const TEST_STR: &str = r#"7/7/7/7/7/7/7 x 0 1; D1 0; D2 0; D3 0; D4 0
    7/7/7/7/7/7/7 o 0 1; D1 0; D2 0; D3 0; D4 0
    x5o/7/7/7/7/7/o5x x 0 1; D1 16; D2 256; D3 6460; D4 155888; D5 4752668
    x5o/7/7/7/7/7/o5x o 0 1; D1 16; D2 256; D3 6460; D4 155888; D5 4752668
    7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1; D1 1; D2 75; D3 249; D4 14270; D5 452980
    7/7/7/7/ooooooo/ooooooo/xxxxxxx o 0 1; D1 75; D2 249; D3 14270; D4 452980
    7/7/7/7/xxxxxxx/xxxxxxx/ooooooo x 0 1; D1 75; D2 249; D3 14270; D4 452980
    7/7/7/7/xxxxxxx/xxxxxxx/ooooooo o 0 1; D1 1; D2 75; D3 249; D4 14270; D5 452980
    7/7/7/2x1o2/7/7/7 x 0 1; D1 23; D2 419; D3 7887; D4 168317; D5 4266992
    7/7/7/2x1o2/7/7/7 o 0 1; D1 23; D2 419; D3 7887; D4 168317; D5 4266992"#;
    
    const GAPPED_TESTS: &str = r#"7/7/7/7/-------/-------/x5o x 0 1; D1 2; D2 4; D3 13; D4 30; D5 73; D6 174
    7/7/7/7/-------/-------/x5o o 0 1; D1 2; D2 4; D3 13; D4 30; D5 73; D6 174
    x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1; D1 14; D2 196; D3 4184; D4 86528; D5 2266352
    x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1; D1 14; D2 196; D3 4184; D4 86528; D5 2266352
    x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1; D1 14; D2 196; D3 4100; D4 83104; D5 2114588
    x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1; D1 14; D2 196; D3 4100; D4 83104; D5 2114588
    x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1; D1 16; D2 256; D3 5948; D4 133264; D5 3639856
    x5o/7/3-3/2-1-2/3-3/7/o5x o 0 1; D1 16; D2 256; D3 5948; D4 133264; D5 3639856"#;

    run_tests("Ataxx Gapless", Ataxx::create(), TEST_STR);
    run_tests("Ataxx Gapped", Ataxx::create(), GAPPED_TESTS);
}
