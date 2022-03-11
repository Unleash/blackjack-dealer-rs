use criterion::{black_box, criterion_group, criterion_main, Criterion};
use logic::{
    card::{Card, Rank, Suit},
    deck_generator::{
        complete_deck, dealer_blackjack, dealer_bust, four_aces, player_blackjack, player_bust,
        shuffle,
    },
};
use strum::IntoEnumIterator;

fn create_hand_benchmark(c: &mut Criterion) {
    c.bench_function("dealer_blackjack", |b| b.iter(|| dealer_blackjack()));
    c.bench_function("dealer_bust", |b| b.iter(|| dealer_bust()));
    c.bench_function("four_aces", |b| b.iter(|| four_aces()));
    c.bench_function("player_blackjack", |b| b.iter(|| player_blackjack()));
    c.bench_function("player_bust", |b| b.iter(|| player_bust()));
    c.bench_function("shuffle", |b| b.iter(|| shuffle()));
}

fn complete_deck_benchmark(c: &mut Criterion) {
    let cards = Suit::iter()
        .flat_map(|s| {
            Rank::iter().map(move |r: Rank| Card {
                value: r,
                suit: s.clone(),
            })
        })
        .collect::<Vec<Card>>();

    c.bench_function("dealer_blackjack:10", |b| {
        b.iter(|| complete_deck(black_box(cards[0..10].to_vec().clone())))
    });
    c.bench_function("dealer_blackjack:20", |b| {
        b.iter(|| complete_deck(black_box(cards[0..20].to_vec().clone())))
    });
    c.bench_function("dealer_blackjack:30", |b| {
        b.iter(|| complete_deck(black_box(cards[0..30].to_vec().clone())))
    });
    c.bench_function("dealer_blackjack:40", |b| {
        b.iter(|| complete_deck(black_box(cards[0..40].to_vec().clone())))
    });
    c.bench_function("dealer_blackjack:50", |b| {
        b.iter(|| complete_deck(black_box(cards[0..50].to_vec().clone())))
    });
}

criterion_group!(benches, create_hand_benchmark, complete_deck_benchmark);
criterion_main!(benches);
