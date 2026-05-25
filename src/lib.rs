//! # MadeOnSol — official Rust SDK
//!
//! Solana KOL wallet tracking, Pump.fun deployer intelligence, alpha-wallet scoring,
//! and an all-DEX trade firehose.
//!
//! ## Get an API key
//!
//! Free tier: **200 requests/day, no credit card** at <https://madeonsol.com/pricing>.
//! Paid tiers (PRO $49/mo, ULTRA $149/mo) unlock higher rate limits, sub-hour windows,
//! WebSocket streaming, webhooks, and the all-DEX firehose.
//!
//! All keys start with `msk_`.
//!
//! ## Quick start
//!
//! ```no_run
//! use madeonsol::{MadeOnSol, types::KolFeedParams};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = std::env::var("MADEONSOL_API_KEY")?;
//! let client = MadeOnSol::new(api_key)?;
//!
//! let feed = client
//!     .kol
//!     .feed(&KolFeedParams { limit: Some(10), ..Default::default() })
//!     .await?;
//!
//! for trade in feed.trades {
//!     println!("{:?} bought {:?}", trade.kol_name, trade.token_symbol);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Namespaces
//!
//! - [`MadeOnSol::kol`] — KOL feed, leaderboard, coordination, PnL, trending tokens, alerts, scout leaderboard
//! - [`MadeOnSol::deployer`] — Pump.fun deployer leaderboard, alerts, trajectory
//! - [`MadeOnSol::alpha`] — alpha-wallet leaderboard, profiles, cap tables, buyer quality
//! - [`MadeOnSol::wallet_tracker`] — track arbitrary Solana wallets (watchlist)
//! - [`MadeOnSol::wallet`] — universal wallet stats, FIFO PnL, open positions, paginated trades (PRO+)
//! - [`MadeOnSol::coordination_alerts`] — push alerts on coordinated buying (PRO/ULTRA)
//! - [`MadeOnSol::price_alerts`] — price-drop / recovery alert rules CRUD (PRO/ULTRA)
//! - [`MadeOnSol::tools`] — Solana tool directory search
//! - [`MadeOnSol::stream`] — WebSocket streaming token issuance
//! - [`MadeOnSol::webhooks`] — webhook CRUD (PRO/ULTRA)
//!
//! Full API reference: <https://madeonsol.com/api-docs>

#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

mod client;
pub mod api;
pub mod error;
pub mod types;

use std::sync::Arc;

use crate::api::{
    alpha::Alpha, coordination_alerts::CoordinationAlerts, deployer::Deployer,
    first_touch_subscriptions::FirstTouchSubscriptions, kol::Kol, me::Me,
    price_alerts::PriceAlerts, stream::Stream, token::Token, tools::Tools,
    wallet::Wallet, wallet_tracker::WalletTracker, webhooks::Webhooks,
};
use crate::client::HttpCore;
use crate::error::{MadeOnSolError, Result};

pub use crate::error::MadeOnSolError as Error;

/// MadeOnSol API client.
///
/// Construct with [`MadeOnSol::new`] and a `msk_…` API key, then access the
/// namespaced sub-clients ([`kol`](Self::kol), [`deployer`](Self::deployer), etc.).
///
/// Cheap to clone — internal HTTP state is reference-counted.
///
/// # Example
///
/// ```no_run
/// use madeonsol::MadeOnSol;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = MadeOnSol::new(std::env::var("MADEONSOL_API_KEY")?)?;
/// let stats = client.deployer.stats().await?;
/// println!("{} deployers tracked", stats.tracked_count);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct MadeOnSol {
    /// KOL wallet tracking endpoints.
    pub kol: Kol,
    /// Pump.fun deployer intelligence endpoints.
    pub deployer: Deployer,
    /// Alpha wallet intelligence: leaderboard, profiles, cap tables, buyer quality.
    pub alpha: Alpha,
    /// Token intelligence — comprehensive per-mint snapshot + batch lookups.
    pub token: Token,
    /// Account self-inspection — tier, quota, feature usage (v0.8).
    pub me: Me,
    /// Wallet tracker: watchlist CRUD, trades, summary.
    pub wallet_tracker: WalletTracker,
    /// Universal wallet endpoints — stats, FIFO PnL, open positions, paginated trades for any Solana wallet. PRO+.
    pub wallet: Wallet,
    /// Coordination alert rules CRUD (v1.1) — PRO/ULTRA.
    pub coordination_alerts: CoordinationAlerts,
    /// First-touch webhook subscriptions CRUD — ULTRA only. Use `kol.first_touches()` for read-only queries.
    pub first_touch_subscriptions: FirstTouchSubscriptions,
    /// Price-drop / recovery alert rules CRUD (v1.9) — PRO/ULTRA.
    pub price_alerts: PriceAlerts,
    /// Solana tool directory search.
    pub tools: Tools,
    /// WebSocket streaming token issuance.
    pub stream: Stream,
    /// Webhook management (PRO/ULTRA).
    pub webhooks: Webhooks,
}

impl MadeOnSol {
    /// Construct a new client.
    ///
    /// `api_key` must start with `msk_`. Get a free key (200 req/day, no card)
    /// at <https://madeonsol.com/pricing>.
    ///
    /// # Errors
    ///
    /// Returns [`MadeOnSolError::MissingApiKey`] if the key is empty or missing the
    /// `msk_` prefix. The error message includes the signup URL so end users know
    /// where to go.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if !api_key.starts_with("msk_") {
            // Print to stderr too — a bare Err can be swallowed and the user
            // never sees the link to /pricing.
            eprintln!(
                "\n[madeonsol] Missing or invalid API key.\n\
                 → Get a free key (200 req/day, no card) at https://madeonsol.com/pricing\n\
                 → Then: madeonsol::MadeOnSol::new(std::env::var(\"MADEONSOL_API_KEY\")?)?\n"
            );
            return Err(MadeOnSolError::MissingApiKey);
        }

        let core = Arc::new(HttpCore::new(api_key));
        Ok(Self {
            kol: Kol { core: Arc::clone(&core) },
            deployer: Deployer { core: Arc::clone(&core) },
            alpha: Alpha { core: Arc::clone(&core) },
            token: Token { core: Arc::clone(&core) },
            me: Me { core: Arc::clone(&core) },
            wallet_tracker: WalletTracker { core: Arc::clone(&core) },
            wallet: Wallet { core: Arc::clone(&core) },
            coordination_alerts: CoordinationAlerts { core: Arc::clone(&core) },
            first_touch_subscriptions: FirstTouchSubscriptions { core: Arc::clone(&core) },
            price_alerts: PriceAlerts { core: Arc::clone(&core) },
            tools: Tools { core: Arc::clone(&core) },
            stream: Stream { core: Arc::clone(&core) },
            webhooks: Webhooks { core },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_api_key() {
        let err = MadeOnSol::new("").unwrap_err();
        assert!(matches!(err, MadeOnSolError::MissingApiKey));
    }

    #[test]
    fn rejects_wrong_prefix() {
        let err = MadeOnSol::new("sk_live_abc").unwrap_err();
        assert!(matches!(err, MadeOnSolError::MissingApiKey));
    }

    #[test]
    fn accepts_valid_prefix() {
        let client = MadeOnSol::new("msk_test_abcdef").unwrap();
        // Smoke test — namespaces exist and the client clones cheaply.
        let _cloned = client.clone();
    }
}
