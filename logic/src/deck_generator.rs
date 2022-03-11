use std::collections::HashSet;
use std::iter::FromIterator;

use crate::card::{Card, Rank, Suit};
use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

lazy_static! {
    pub static ref DECK: Vec<Card> = Suit::iter()
        .flat_map(|s| Rank::iter().map(move |r: Rank| Card {
            value: r,
            suit: s.clone()
        }))
        .collect::<Vec<Card>>();
    static ref DECKSET: HashSet<Card> = HashSet::from_iter(DECK.clone());
}

type Deck = Vec<Card>;

pub fn shuffle() -> Deck {
    let mut rng = thread_rng();
    let mut deck_copy = DECK.clone();
    deck_copy.shuffle(&mut rng);
    deck_copy
}

pub fn complete_deck(front_of_deck: Vec<Card>) -> Deck {
    let mut leaders = front_of_deck.clone();
    leaders.sort();
    let mut leader_iterator = leaders.iter();
    let mut current_leader_index = leader_iterator.next().map(|card| card.standard_index());

    let deck_without_leaders = DECK
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            if current_leader_index == Some(*index) {
                current_leader_index = leader_iterator.next().map(|card| card.standard_index());
                false
            } else {
                true
            }
        })
        .map(|(_, card)| card.clone());

    let mut deck = front_of_deck.clone();
    deck.extend(deck_without_leaders);
    deck
}

pub fn four_aces() -> Deck {
    let four_aces: Vec<Card> = Suit::iter()
        .map(|s| Card {
            suit: s,
            value: Rank::Ace,
        })
        .collect::<Vec<Card>>();
    complete_deck(four_aces)
}

pub fn player_blackjack() -> Deck {
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

pub fn dealer_blackjack() -> Deck {
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

pub fn player_bust() -> Deck {
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

pub fn dealer_bust() -> Deck {
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

    #[test]
    fn complete_deck_returns_complete_and_correct_deck() {
        let four_aces = [
            Card::from_str("SA").unwrap(),
            Card::from_str("HA").unwrap(),
            Card::from_str("CA").unwrap(),
            Card::from_str("DA").unwrap(),
        ];

        let new_deck = complete_deck(four_aces.to_vec());
        let unique_cards: HashSet<Card> = HashSet::from_iter(new_deck.clone());
        assert_eq!(new_deck.len(), 52);
        assert_eq!(unique_cards.len(), 52);
    }
}
