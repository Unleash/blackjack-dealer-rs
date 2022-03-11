#![warn(clippy::all)]

use std::collections::HashSet;

use lazy_static::lazy_static;
use prometheus::Registry;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};
use warp::http::Response;
use warp::Filter;
use warp_prometheus::Metrics;
mod card;
use card::{Card, Rank, Suit};

lazy_static! {
    static ref DECK: Vec<Card> = Suit::iter()
        .flat_map(|s| Rank::iter().map(move |r: Rank| Card {
            value: r,
            suit: s.clone()
        }))
        .collect::<Vec<Card>>();
    static ref DECKSET: HashSet<Card> = HashSet::from_iter(DECK.clone());
    static ref REGISTRY: Registry = Registry::new();
}

type Deck = Vec<Card>;

fn shuffle() -> Deck {
    let mut rng = thread_rng();
    let mut deck_copy = DECK.clone();
    deck_copy.shuffle(&mut rng);
    deck_copy
}

fn complete_deck(front_of_deck: Vec<Card>) -> Deck {
    let mut rest_of_deck = DECKSET.clone();
    for c in &front_of_deck {
        rest_of_deck.remove(c);
    }
    let mut cards = front_of_deck.clone();
    cards.extend(rest_of_deck.into_iter());
    cards
}

fn four_aces() -> Deck {
    let four_aces: Vec<Card> = Suit::iter()
        .map(|s| Card {
            suit: s,
            value: Rank::Ace,
        })
        .collect::<Vec<Card>>();
    complete_deck(four_aces)
}

fn player_blackjack() -> Deck {
    let player_blackjack = [
        Card {
            suit: Suit::Spades,
            value: Rank::Ace,
        },
        Card {
            suit: Suit::Hearts,
            value: Rank::Eight,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Jack,
        },
    ];
    complete_deck(player_blackjack.to_vec())
}

fn dealer_blackjack() -> Deck {
    let dealer_blackjack = [
        Card {
            suit: Suit::Spades,
            value: Rank::Five,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Ace,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Nine,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Jack,
        },
    ];
    complete_deck(dealer_blackjack.to_vec())
}

fn player_bust() -> Deck {
    let player_bust = [
        Card {
            suit: Suit::Spades,
            value: Rank::Five,
        },
        Card {
            suit: Suit::Hearts,
            value: Rank::Two,
        },
        Card {
            suit: Suit::Diamonds,
            value: Rank::Five,
        },
        Card {
            suit: Suit::Clubs,
            value: Rank::Nine,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Six,
        },
        Card {
            suit: Suit::Diamonds,
            value: Rank::King,
        },
    ];
    complete_deck(player_bust.to_vec())
}

fn dealer_bust() -> Deck {
    let dealer_bust = [
        Card {
            suit: Suit::Spades,
            value: Rank::Five,
        },
        Card {
            suit: Suit::Hearts,
            value: Rank::Two,
        },
        Card {
            suit: Suit::Diamonds,
            value: Rank::Five,
        },
        Card {
            suit: Suit::Clubs,
            value: Rank::Nine,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Six,
        },
        Card {
            suit: Suit::Diamonds,
            value: Rank::Four,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::Eight,
        },
        Card {
            suit: Suit::Spades,
            value: Rank::King,
        },
    ];
    complete_deck(dealer_bust.to_vec())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let path_includes = vec![
        "shuffle".into(),
        "fouraces".into(),
        "playerblackjack".into(),
        "dealerblackjack".into(),
    ];
    let metrics = Metrics::new(&REGISTRY, &path_includes);
    let logger = warp::log("unleash-blackjack");

    let shuffle = warp::path!("shuffle").and(warp::get()).map(|| {
        let shuffled_deck = shuffle();
        warp::reply::json(&shuffled_deck)
    });
    let fouraces = warp::path!("fouraces").and(warp::get()).map(|| {
        let four_aces = four_aces();
        warp::reply::json(&four_aces)
    });
    let playerblackjack = warp::path!("playerblackjack").and(warp::get()).map(|| {
        let player_twentyone = player_blackjack();
        warp::reply::json(&player_twentyone)
    });
    let dealerblackjack = warp::path!("dealerblackjack").and(warp::get()).map(|| {
        let player_twentyone = dealer_blackjack();
        warp::reply::json(&player_twentyone)
    });

    let playerbust = warp::path!("playerbust").and(warp::get()).map(|| {
        let playerbust = player_bust();
        warp::reply::json(&playerbust)
    });

    let dealerbust = warp::path!("dealerbust").and(warp::get()).map(|| {
        let dealerbust = dealer_bust();
        warp::reply::json(&dealerbust)
    });

    let metrics_route = warp::path!("metrics").and(warp::get()).map(|| {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();

        let mut buffer = Vec::new();
        if let Err(_e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
            log::error!("could not encode custom metrics");
        };
        let mut res = match String::from_utf8(buffer.clone()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("custom metrics could not be from_utf8'd: {}", e);
                String::default()
            }
        };
        buffer.clear();
        let mut buffer = Vec::new();
        if let Err(_e) = encoder.encode(&prometheus::gather(), &mut buffer) {
            log::error!("could not encode prometheus metrics");
        };
        let res_custom = match String::from_utf8(buffer.clone()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("prometheus metrics could not be from_utf8'd: {}", e);
                String::default()
            }
        };
        buffer.clear();

        res.push_str(&res_custom);
        Response::builder().body(res)
    });

    let health = warp::path!("health")
        .and(warp::get())
        .map(|| Response::builder().body("OK"));

    let routes = warp::any()
        .and(
            shuffle
                .or(fouraces)
                .or(playerblackjack)
                .or(dealerblackjack)
                .or(playerbust)
                .or(dealerbust)
                .or(metrics_route)
                .or(health),
        )
        .with(logger)
        .with(warp::log::custom(move |info| metrics.http_metrics(info)));

    let addr = ([0, 0, 0, 0], 1337);
    warp::serve(routes).run(addr).await;
}

#[cfg(test)]
mod blackjack {
    use std::str::FromStr;

    use super::*;

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

    #[test]
    fn four_aces_returns_four_aces_as_first_four_cards() {
        let f = four_aces();
        let four_aces = [
            Card::from_str("SA").unwrap(),
            Card::from_str("HA").unwrap(),
            Card::from_str("CA").unwrap(),
            Card::from_str("DA").unwrap(),
        ];
        assert_eq!(four_aces, f.chunks(4).next().unwrap())
    }

    #[test]
    fn player_blackjack_deals_blackjack_to_player() {
        let b = player_blackjack();
        let first_player_card = b.get(0).unwrap();
        let second_player_card = b.get(2).unwrap();

        assert_eq!(first_player_card, &Card::from_str("SA").unwrap());
        assert_eq!(second_player_card, &Card::from_str("SJ").unwrap())
    }
    #[test]
    fn dealer_blackjack_deals_a_blackjack_to_dealer() {
        let b = dealer_blackjack();
        let first_dealer_card = b.get(1).unwrap();
        let second_dealer_card = b.get(3).unwrap();

        assert_eq!(first_dealer_card, &Card::from_str("SA").unwrap());
        assert_eq!(second_dealer_card, &Card::from_str("SJ").unwrap())
    }
}
