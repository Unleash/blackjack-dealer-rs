#![warn(clippy::all)]

use std::collections::HashSet;
use std::str::FromStr;

use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use warp::Filter;

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, EnumString, Eq, PartialEq, Hash)]
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

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, EnumString, Eq, PartialEq, Hash)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
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
    static ref DECKSET: HashSet<Card> = HashSet::from_iter(DECK.clone());
}

type Deck = Vec<Card>;

fn shuffle() -> Deck {
    let mut rng = thread_rng();
    let mut deck_copy = DECK.clone();
    deck_copy.shuffle(&mut rng);
    deck_copy
}

fn four_aces() -> Deck {
    let mut rest_of_deck = DECKSET.clone();
    let mut four_aces: Vec<Card> = Suit::iter()
        .map(|s| {
            let c = Card {
                suit: s,
                value: Rank::Ace,
            };
            rest_of_deck.remove(&c);
            c.clone()
        })
        .collect::<Vec<Card>>();
    four_aces.extend(rest_of_deck.into_iter());
    four_aces
}

#[tokio::main]
async fn main() {
    let shuffle = warp::path!("shuffle").map(|| {
        let shuffled_deck = shuffle();
        warp::reply::json(&shuffled_deck)
    });
    let fouraces = warp::path!("fouraces").map(|| {
        let four_aces = four_aces();
        warp::reply::json(&four_aces)
    });

    let routes = warp::get().and(shuffle.or(fouraces));

    warp::serve(routes).run(([127, 0, 0, 1], 1337)).await;
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if let Some(suit) = {
            match chars.next() {
                Some('S') => Some(Suit::Spades),
                Some('D') => Some(Suit::Diamonds),
                Some('H') => Some(Suit::Hearts),
                Some('C') => Some(Suit::Clubs),
                _ => None,
            }
        } {
            if let Some(rank) = {
                match chars.next() {
                    Some('2') => Some(Rank::Two),
                    Some('3') => Some(Rank::Three),
                    Some('4') => Some(Rank::Four),
                    Some('5') => Some(Rank::Five),
                    Some('6') => Some(Rank::Six),
                    Some('7') => Some(Rank::Seven),
                    Some('8') => Some(Rank::Eight),
                    Some('9') => Some(Rank::Nine),
                    Some('1') => Some(Rank::Ten),
                    Some('J') => Some(Rank::Jack),
                    Some('Q') => Some(Rank::Queen),
                    Some('K') => Some(Rank::King),
                    Some('A') => Some(Rank::Ace),
                    _ => None,
                }
            } {
                Result::Ok(Card { suit, value: rank })
            } else {
                Result::Err(())
            }
        } else {
            Result::Err(())
        }
    }
}

#[cfg(test)]
#[test]
fn four_aces_returns_four_aces_as_first_four_cards() {
    let f = four_aces();
    let four_aces: [Card; 4] = [
        Card::from_str("SA").unwrap(),
        Card::from_str("HA").unwrap(),
        Card::from_str("CA").unwrap(),
        Card::from_str("DA").unwrap(),
    ];
    assert_eq!(four_aces, f.chunks(4).next().unwrap())
}
