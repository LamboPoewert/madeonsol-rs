//! Print the latest KOL buys.
//!
//! Run with:
//! ```sh
//! MADEONSOL_API_KEY=msk_... cargo run --example kol_feed
//! ```
//!
//! Get a free API key (200 req/day, no card) at https://madeonsol.com/developer.

use madeonsol::{
    types::{KolAction, KolFeedParams},
    MadeOnSol,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("MADEONSOL_API_KEY")
        .expect("set MADEONSOL_API_KEY — get a free one at https://madeonsol.com/developer");

    let client = MadeOnSol::new(api_key)?;

    let feed = client
        .kol
        .feed(&KolFeedParams {
            limit: Some(10),
            action: Some(KolAction::Buy),
            ..Default::default()
        })
        .await?;

    println!("Latest {} KOL buys:\n", feed.trades.len());
    for t in feed.trades {
        println!(
            "  {:>20}  bought  {:<10}  for {:>6.2} SOL",
            t.kol_name.unwrap_or_else(|| t.wallet_address[..6].to_string()),
            t.token_symbol.unwrap_or_else(|| "?".to_string()),
            t.sol_amount,
        );
    }

    Ok(())
}
