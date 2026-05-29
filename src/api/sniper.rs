use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Deshred pre-confirm pump.fun sniper feed. PRO + ULTRA.
///
/// Deploys are reconstructed from shred-level ("deshred") data and surface
/// ~500ms before the chain confirms them — the fastest path to a new pump.fun
/// launch. PRO is curated to elite/good deployers; ULTRA sees every tier and can
/// maintain a custom deployer watchlist.
///
/// These methods are for catch-up / backtesting and watchlist management. For
/// **live** push, use the `sniper:deploy` webhook event, the `sniper:deploys`
/// WebSocket channel, or `/alert sniper` in the Telegram bot.
#[derive(Debug, Clone)]
pub struct Sniper {
    pub(crate) core: Arc<HttpCore>,
}

impl Sniper {
    /// Newest-first deshred deploy feed. PRO sees elite/good deployers; ULTRA sees all.
    /// Set `watchlist: Some(true)` (ULTRA) to narrow to your custom deployer watchlist.
    pub async fn recent(&self, params: &SniperRecentParams) -> Result<SniperRecentResponse> {
        self.core.get("/sniper/recent", params).await
    }

    /// Deshred deploys filtered to a single deployer wallet. ULTRA only.
    pub async fn by_deployer(
        &self,
        wallet: &str,
        params: &SniperByDeployerParams,
    ) -> Result<SniperByDeployerResponse> {
        self.core
            .get(&format!("/sniper/by-deployer/{}", wallet), params)
            .await
    }

    /// List your custom deployer watchlist (ULTRA, max 50).
    pub async fn watchlist(&self) -> Result<SniperWatchlistResponse> {
        self.core.get("/sniper/watchlist", &()).await
    }

    /// Add one (`wallet`) or many (`wallets`) deployers to your watchlist. ULTRA only.
    pub async fn add_to_watchlist(
        &self,
        params: &SniperWatchlistAddParams,
    ) -> Result<SniperWatchlistAddResponse> {
        self.core.post_json("/sniper/watchlist", params).await
    }

    /// Remove a deployer from your watchlist. ULTRA only.
    pub async fn remove_from_watchlist(
        &self,
        wallet: &str,
    ) -> Result<SniperWatchlistRemoveResponse> {
        self.core
            .delete(&format!("/sniper/watchlist/{}", wallet))
            .await
    }
}
