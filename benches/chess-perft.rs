use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monster_chess::{board::Board, games::chess::Chess};

fn startpos(depth: u32) {
    let game = Chess::create();
    let mut board = Board::new(
        &game,
        (8, 8),
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );

    board.sub_perft(depth);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("startpos", |b| b.iter(|| startpos(4)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
