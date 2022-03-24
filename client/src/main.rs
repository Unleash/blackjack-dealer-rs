use clap::Parser;
use client::play_blackjack;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct ClientArgs {
    #[clap(
        short,
        long,
        default_value = "https://sandbox.getunleash.io/blackjack/shuffle"
    )]
    url: String,
    #[clap(short, long, default_value = "Sam")]
    player_name: String,
}

#[tokio::main]
async fn main() {
    let client_args = ClientArgs::parse();
    let result = play_blackjack(client_args.url, client_args.player_name).await;
    println!("{:#?}", result)
}
