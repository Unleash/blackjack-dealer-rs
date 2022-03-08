#![warn(clippy::all)]

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use warp::Filter;

#[derive(Serialize, Deserialize, Clone, EnumIter, EnumString)]
enum Suit {
    #[strum(serialize = "S")]
    Spades,
    #[strum(serialize = "H")]
    Hearts,
    #[strum(serialize = "C")]
    Clubs,
    #[strum(serialize = "D")]
    Diamonds,
}

#[derive(Serialize, Deserialize, Clone, EnumIter, EnumString)]
enum Rank {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "10")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    suit: Suit,
    value: Rank,
}

lazy_static! {
    static ref DECK: Vec<Card> = Suit::iter()
        .flat_map(|s| Rank::iter().map(move |r: Rank| Card {
            value: r,
            suit: s.clone()
        }))
        .collect::<Vec<Card>>();
}

type Deck = Vec<Card>;

fn shuffle() -> Deck {
    let mut rng = thread_rng();
    let mut deck_copy = DECK.clone();
    deck_copy.shuffle(&mut rng);
    deck_copy
}

#[tokio::main]
async fn main() {
    let routes = warp::path!("shuffle").and(warp::get()).map(|| {
        let shuffled_deck = shuffle();
        warp::reply::json(&shuffled_deck)
    });

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;
}
