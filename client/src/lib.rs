use logic::card::{Card, Rank};
use reqwest::get;
use serde::{Deserialize, Serialize};
type Deck = Vec<Card>;

async fn fetch_deck(url: String) -> reqwest::Result<Deck> {
    get(url).await?.json::<Deck>().await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerResult {
    name: String,
    hand: Deck,
    score: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResult {
    deck: Deck,
    players: Vec<PlayerResult>,
    winner: String,
}

pub trait ToInt {
    fn to_int(&self) -> u8;
}

impl ToInt for Card {
    fn to_int(&self) -> u8 {
        self.value.to_int()
    }
}

impl ToInt for Rank {
    fn to_int(&self) -> u8 {
        match &self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ace => 11,
            _ => 10,
        }
    }
}

trait Blackjack<S: Scorable = Self> {
    fn has_blackjack(&self) -> bool;
}
trait Scorable {
    fn score(&self) -> u8;
}
trait Bust<S: Scorable = Self> {
    fn is_bust(&self) -> bool;
}

trait Draw {
    fn hit_me(&self, other_score: u8) -> bool;
}

trait ToPlayerResult {
    fn to_result(&self) -> PlayerResult;
}

pub struct Player {
    name: String,
    hand: Vec<Card>,
    strategy: Box<dyn Fn(u8, u8) -> bool>,
}
impl ToPlayerResult for Player {
    fn to_result(&self) -> PlayerResult {
        PlayerResult {
            name: self.name.clone(),
            hand: self.hand.clone(),
            score: self.score(),
        }
    }
}
impl Scorable for Player {
    fn score(&self) -> u8 {
        self.hand.iter().map(|x| x.to_int()).sum()
    }
}
impl Draw for Player {
    fn hit_me(&self, other_score: u8) -> bool {
        (self.strategy)(self.score(), other_score)
    }
}
impl Blackjack for Player {
    fn has_blackjack(&self) -> bool {
        self.hand.len() == 2 && self.score() == 21
    }
}
impl Bust for Player {
    fn is_bust(&self) -> bool {
        self.score() > 21
    }
}

fn player_wins(original: Deck, player: Player, dealer: Player) -> GameResult {
    GameResult {
        winner: player.name.clone(),
        players: vec![player.to_result(), dealer.to_result()],
        deck: original,
    }
}

fn dealer_wins(original: Deck, player: Player, dealer: Player) -> GameResult {
    GameResult {
        winner: dealer.name.clone(),
        players: vec![player.to_result(), dealer.to_result()],
        deck: original,
    }
}

fn play_game(deck: Deck, player_name: String) -> GameResult {
    println!("Playing as {:?}", player_name);
    println!("{:#?}", deck);
    let original = deck.clone();
    let mut playing_deck = deck.clone();
    let starting_hand = vec![playing_deck.remove(0), playing_deck.remove(0)];
    let mut player = Player {
        name: player_name,
        hand: starting_hand,
        strategy: Box::new(|own_score, _| own_score < 17),
    };
    let dealer_hand = vec![playing_deck.remove(0), playing_deck.remove(0)];
    let mut dealer = Player {
        name: "Dealer".into(),
        hand: dealer_hand,
        strategy: Box::new(|own_score, player_score| own_score <= player_score),
    };
    if dealer.has_blackjack() {
        return dealer_wins(original, player, dealer);
    } else if player.has_blackjack() {
        return player_wins(original, player, dealer);
    }
    while (player.hit_me(dealer.score())) {
        player.hand.push(playing_deck.remove(0));
    }
    if (player.is_bust()) {
        return dealer_wins(original, player, dealer);
    }
    let p_score = player.score();
    while (dealer.hit_me(p_score)) {
        dealer.hand.push(playing_deck.remove(0));
    }
    if (dealer.is_bust()) {
        return player_wins(original, player, dealer);
    }
    dealer_wins(original, player, dealer)
}

pub async fn play_blackjack(url: String, player_name: String) -> GameResult {
    let deck = fetch_deck(url).await.expect("Could not parse deck");
    play_game(deck, player_name)
}
