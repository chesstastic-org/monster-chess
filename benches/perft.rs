use criterion::{criterion_group, criterion_main, Criterion};
use monster_chess::{games::{chess::Chess, ataxx::Ataxx}, board::game::Game};

fn startpos(game: Game<1>, depth: u32) {
    let mut board = game.default();
    board.perft(depth, true);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chess", |b| b.iter(|| startpos(Chess::create(), 4)));
    c.bench_function("ataxx", |b| b.iter(|| startpos(Ataxx::create(), 4)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
