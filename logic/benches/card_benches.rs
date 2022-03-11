use criterion::{criterion_group, criterion_main, Criterion};
use logic::deck_generator::{
    dealer_blackjack, dealer_bust, four_aces, player_blackjack, player_bust, shuffle,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dealer_blackjack", |b| b.iter(|| dealer_blackjack()));
    c.bench_function("dealer_bust", |b| b.iter(|| dealer_bust()));
    c.bench_function("four_aces", |b| b.iter(|| four_aces()));
    c.bench_function("player_blackjack", |b| b.iter(|| player_blackjack()));
    c.bench_function("player_bust", |b| b.iter(|| player_bust()));
    c.bench_function("shuffle", |b| b.iter(|| shuffle()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
