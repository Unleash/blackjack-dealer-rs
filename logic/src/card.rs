use std::cmp::Ordering;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, EnumString, Eq, PartialEq, Hash)]
pub enum Suit {
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
pub enum Rank {
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

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.standard_index().cmp(&other.standard_index())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}

impl Card {
    pub fn standard_index(&self) -> usize {
        ((self.suit.clone() as u8) * 13 + self.value.clone() as u8) as usize
    }
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
mod tests {
    use crate::deck_generator::DECK;

    use super::*;

    #[test]
    fn card_indexes_on_standard_deck() {
        let card = Card {
            suit: Suit::Clubs,
            value: Rank::Five,
        };

        let dummy = Card {
            suit: Suit::Clubs,
            value: Rank::Six,
        };

        let drawn_card = DECK[card.standard_index()].clone();
        assert_eq!(drawn_card, card);
        assert_ne!(drawn_card, dummy);
    }
}
