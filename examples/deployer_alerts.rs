//! Stream the latest elite-tier Pump.fun deployer launches.
//!
//! Run with:
//! ```sh
//! MADEONSOL_API_KEY=msk_... cargo run --example deployer_alerts
//! ```
//!
//! NOTE: the `tier` filter is PRO/ULTRA only. On the free tier this still works,
//! it just returns alerts across all tiers.
//!
//! Get a free API key (200 req/day, no card) at https://madeonsol.com/developer.

use madeonsol::{
    types::{DeployerAlertsParams, DeployerTier},
    MadeOnSol,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("MADEONSOL_API_KEY")
        .expect("set MADEONSOL_API_KEY — get a free one at https://madeonsol.com/developer");

    let client = MadeOnSol::new(api_key)?;

    let alerts = client
        .deployer
        .alerts(&DeployerAlertsParams {
            limit: Some(5),
            tier: Some(DeployerTier::Elite),
            ..Default::default()
        })
        .await?;

    println!("Latest {} elite deployer launches:\n", alerts.alerts.len());
    for a in alerts.alerts {
        println!(
            "  [{:?}] {:<10}  {:<22}  bonding rate {:.1}%",
            a.deployers.tier,
            a.token_symbol.unwrap_or_else(|| "?".to_string()),
            a.token_name.unwrap_or_else(|| "?".to_string()),
            a.deployers.bonding_rate.unwrap_or(0.0) * 100.0,
        );
    }

    Ok(())
}
