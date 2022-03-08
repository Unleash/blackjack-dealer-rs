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
    #[serde(rename = "SPADES")]
    Spades,
    #[serde(rename = "HEARTS")]
    Hearts,
    #[serde(rename = "CLUBS")]
    Clubs,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
}

#[derive(Serialize, Deserialize, Clone, EnumIter, EnumString)]
enum Rank {
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "3")]
    Three,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "6")]
    Six,
    #[serde(rename = "7")]
    Seven,
    #[serde(rename = "8")]
    Eight,
    #[serde(rename = "9")]
    Nine,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "J")]
    Jack,
    #[serde(rename = "Q")]
    Queen,
    #[serde(rename = "K")]
    King,
    #[serde(rename = "A")]
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
