use super::game::Game;

fn get_time_ms() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}


pub struct FENTest<'a> {
    fen: &'a str,
    perft_counts: Vec<u64>
}

pub fn parse_tests<'a>(tests: &str) -> Vec<FENTest<'a>> {
    tests.split("\n").map(|test| {
        let strs = test.split(";").collect::<Vec<_>>();
        let fen = strs[0].trim();
        let perft_counts = strs[1..].iter().map(|str| str.split(" ").nth(1).unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
        FENTest {
            fen,
            perft_counts
        }
    }).collect::<Vec<_>>()
}

pub fn run_tests(game: Game, tests: &str) {
    let tests = parse_tests(tests);
    let test_count = tests.len();

    for depth in 1..100 {
        if depth == 1 {
            println!("Testing depth {depth}...");
        } else {
            println!("Testing depth {depth}...");
        }
        let mut tests_completed = 0;
        let mut start = get_time_ms();
        let mut nodes = 0;
        for (ind, test) in tests.iter().enumerate() {
            if depth > test.perft_counts.len() {
                continue;
            }

            let chess = Chess::create();
            let mut board = chess.from_fen(test.fen);
            board.assert_perft(depth as u32, test.perft_counts[depth - 1]);
            tests_completed += 1;

            nodes += test.perft_counts[depth - 1];

            let end = get_time_ms();
            if (end - start) > 400 {
                println!("  {}% complete ({nodes} nodes so far)", (((ind as f64) / (test_count as f64)) * 100.0) as u64);
                start = get_time_ms();
            }
        }
        if tests_completed == 0 {
            println!("No tests found for depth {depth}, ending!");
            return;
        }
        print!("All tests for depth {depth} have completed ({nodes} nodes searched.) ");
    }
}
