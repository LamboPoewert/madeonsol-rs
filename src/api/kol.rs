use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// KOL wallet tracking endpoints — feed, leaderboard, coordination, profiles, alerts.
#[derive(Debug, Clone)]
pub struct Kol {
    pub(crate) core: Arc<HttpCore>,
}

impl Kol {
    /// Live feed of trades made by tracked KOL wallets.
    pub async fn feed(&self, params: &KolFeedParams) -> Result<KolFeedResponse> {
        self.core.get("/kol/feed", params).await
    }

    /// KOL PnL leaderboard ranked by realized profit.
    pub async fn leaderboard(
        &self,
        params: &KolLeaderboardParams,
    ) -> Result<KolLeaderboardResponse> {
        self.core.get("/kol/leaderboard", params).await
    }

    /// Full profile for a single KOL wallet.
    pub async fn wallet(
        &self,
        wallet: &str,
        params: &KolWalletParams,
    ) -> Result<KolWalletProfile> {
        self.core.get(&format!("/kol/{}", wallet), params).await
    }

    /// Detect tokens where multiple KOLs are buying simultaneously.
    pub async fn coordination(
        &self,
        params: &KolCoordinationParams,
    ) -> Result<KolCoordinationResponse> {
        self.core.get("/kol/coordination", params).await
    }

    /// KOL buy/sell activity for a specific token mint.
    pub async fn token(&self, mint: &str) -> Result<KolTokenActivity> {
        self.core
            .get(&format!("/kol/tokens/{}", mint), &())
            .await
    }

    /// KOL affinity pairs — wallets that frequently co-trade the same tokens.
    pub async fn pairs(&self, params: &KolPairsParams) -> Result<KolPairsResponse> {
        self.core.get("/kol/pairs", params).await
    }

    /// KOL entry/exit timing profile — hold duration, exit speed, activity hours.
    pub async fn timing(
        &self,
        wallet: &str,
        params: &KolTimingParams,
    ) -> Result<KolTimingResponse> {
        self.core
            .get(&format!("/kol/{}/timing", wallet), params)
            .await
    }

    /// Tokens with accelerating KOL buy interest.
    pub async fn hot_tokens(
        &self,
        params: &KolHotTokensParams,
    ) -> Result<KolHotTokensResponse> {
        self.core.get("/kol/tokens/hot", params).await
    }

    /// Deep per-wallet PnL breakdown — equity curve, profit factor, max drawdown.
    pub async fn pnl(&self, wallet: &str, params: &KolPnlParams) -> Result<KolPnlResponse> {
        self.core
            .get(&format!("/kol/{}/pnl", wallet), params)
            .await
    }

    /// Tokens ranked by KOL buy volume across multiple time windows.
    pub async fn trending_tokens(
        &self,
        params: &KolTrendingParams,
    ) -> Result<KolTrendingResponse> {
        self.core.get("/kol/tokens/trending", params).await
    }

    /// Ranked list of KOL buyers for a token, ordered by first-buy timestamp.
    pub async fn token_entry_order(
        &self,
        mint: &str,
        params: &KolEntryOrderParams,
    ) -> Result<KolEntryOrderResponse> {
        self.core
            .get(
                &format!("/kol/tokens/{}/entry-order", mint),
                params,
            )
            .await
    }

    /// Side-by-side comparison of 2–5 KOL wallets.
    /// Tier limits: BASIC=2, PRO=4, ULTRA=5.
    pub async fn compare(&self, params: &KolCompareParams) -> Result<KolCompareResponse> {
        let wallets_joined = params.wallets.join(",");
        self.core
            .get("/kol/compare", &[("wallets", wallets_joined)])
            .await
    }

    /// Live feed of notable KOL events — consensus clusters, fresh-launch buys, heating-up wallets.
    pub async fn alerts(&self, params: &KolAlertsParams) -> Result<KolAlertsResponse> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(window) = &params.window {
            query.push(("window", window.clone()));
        }
        if let Some(limit) = params.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(types) = &params.types {
            if !types.is_empty() {
                query.push(("types", types.join(",")));
            }
        }
        self.core.get("/kol/alerts/recent", &query).await
    }

    /// v1.9 — Scout leaderboard: top KOLs ranked by scout score, first-touch frequency,
    /// and swarm attraction rate. ULTRA only.
    pub async fn scout_leaderboard(
        &self,
        params: &ScoutLeaderboardParams,
    ) -> Result<serde_json::Value> {
        self.core.get("/kol/scouts/leaderboard", params).await
    }

    /// v1.9 — Coordination history: past coordination alert fires with token, score,
    /// KOL count. ULTRA only.
    pub async fn coordination_history(
        &self,
        params: &CoordinationHistoryParams,
    ) -> Result<serde_json::Value> {
        self.core.get("/kol/coordination/history", params).await
    }

    /// Recent first-KOL-touch events on tokens — every time a tracked KOL was the first
    /// to buy a given mint. Filterable by scout tier (S/A/B/C from `mv_kol_scout_score`),
    /// KOL winrate, token age, mint suffix, etc.
    ///
    /// Backtest: top scouts attract ≥3 follow-on KOLs within 4h ~50% of the time vs ~14% baseline.
    /// Median lead time before second KOL is 12s — for trading this signal, use the
    /// `kol:first_touches` WebSocket channel rather than polling.
    pub async fn first_touches(
        &self,
        params: &FirstTouchesParams,
    ) -> Result<FirstTouchesResponse> {
        self.core.get("/kol/first-touches", params).await
    }
}
