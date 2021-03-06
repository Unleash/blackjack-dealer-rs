#![warn(clippy::all)]

use lazy_static::lazy_static;
use logic::card::{Card, FromAnswer};
use logic::deck_generator::{
    both_blackjack, complete_deck, dealer_blackjack, dealer_bust, four_aces, player_blackjack,
    player_bust, shuffle, tie21, BlackjackQuery,
};
use logic::error::ErrorMessage;
use prometheus::Registry;
use std::convert::Infallible;
use std::env;
use warp::http::{Response, StatusCode};
use warp::{Filter, Rejection, Reply};
use warp_prometheus::Metrics;

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
}

async fn handle_reject(_r: Rejection) -> Result<impl Reply, Infallible> {
    let json = warp::reply::json(&ErrorMessage {
        code: 400,
        message: "Invalid deck format".into(),
    });

    Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let path_includes = vec![
        "shuffle".into(),
        "fouraces".into(),
        "playerblackjack".into(),
        "dealerblackjack".into(),
        "custom".into(),
        "playerbust".into(),
        "dealerbust".into(),
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

    let both_blackjack = warp::path!("bothblackjack").and(warp::get()).map(|| {
        let both_blackjack = both_blackjack();
        warp::reply::json(&both_blackjack)
    });

    let tie21 = warp::path!("tie21").and(warp::get()).map(|| {
        let tie21 = tie21();
        warp::reply::json(&tie21)
    });

    let customdeck = warp::path!("custom")
        .and(warp::get())
        .and(warp::query::<BlackjackQuery>())
        .map(|q: BlackjackQuery| {
            let cards = q
                .cards
                .split(',')
                .map(Card::from_answer)
                .collect::<Result<Vec<Card>, ()>>()
                .unwrap_or_default();
            let custom = complete_deck(cards);
            warp::reply::json(&custom)
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
                .or(both_blackjack)
                .or(tie21)
                .or(playerbust)
                .or(dealerbust)
                .or(customdeck)
                .or(metrics_route)
                .or(health),
        )
        .recover(handle_reject)
        .with(logger)
        .with(warp::log::custom(move |info| metrics.http_metrics(info)));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "1337".to_string())
        .parse()
        .expect("Must be a number");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
