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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Rank,
}
