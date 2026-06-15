use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Token intelligence endpoints — comprehensive per-mint snapshot and batch lookups.
#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) core: Arc<HttpCore>,
}

impl Token {
    /// Comprehensive per-mint snapshot: price (VWAP), market cap, 24h volume,
    /// deployer reputation, KOL smart-money activity, first_seen_at / age_seconds,
    /// and blacklist status — all in one call.
    ///
    /// **ULTRA** adds individual KOL wallet addresses in `kol_activity.top_buyers[].wallet`.
    pub async fn get(&self, mint: &str) -> Result<TokenResponse> {
        self.core.get(&format!("/token/{}", mint), &()).await
    }

    /// Batch lookup of up to 50 mints. Returns the same per-mint shape as `get()`
    /// in a single round-trip — DB queries batched with `IN(...)`, dex-stream and
    /// RPC fan-outs run in parallel. Roughly 10-20× cheaper than N sequential calls.
    pub async fn batch(&self, mints: Vec<String>) -> Result<TokenBatchResponse> {
        self.core
            .post_json("/token/batch", &MintBatchRequest { mints })
            .await
    }

    /// Batch buyer-quality scoring for up to 50 mints. Shares the same 5-minute
    /// LRU cache as `alpha::buyer_quality(mint)` — already-warm mints return at
    /// near-zero cost. Response includes a `cache_hits` counter.
    pub async fn batch_buyer_quality(
        &self,
        mints: Vec<String>,
    ) -> Result<AlphaBuyerQualityBatchResponse> {
        self.core
            .post_json("/tokens/batch/buyer-quality", &MintBatchRequest { mints })
            .await
    }

    /// v1.9 — KOL consensus on a token: how many KOLs bought/sold, exit rate,
    /// net flow, median entry MC. ULTRA gets individual wallet arrays.
    pub async fn kol_consensus(&self, mint: &str) -> Result<KolConsensusResponse> {
        self.core
            .get(&format!("/tokens/{}/kol-consensus", mint), &())
            .await
    }

    /// v1.9 — Peak MC history for a token: ATH, decline from peak, MC at bond
    /// and at 1h/6h/24h/7d after bond.
    pub async fn peak_history(&self, mint: &str) -> Result<PeakHistoryResponse> {
        self.core
            .get(&format!("/tokens/{}/peak-history", mint), &())
            .await
    }

    /// v0.14 — Transparent 0–100 token rug-risk / safety score (PRO/ULTRA).
    /// Higher means riskier. Returns the overall `risk_score` and `band`
    /// alongside a per-factor breakdown (mint/freeze authority, liquidity,
    /// transfer fee, launch cohort, deployer reputation, blacklist, …) and the
    /// raw `inputs` each factor was derived from — nothing is opaque.
    pub async fn risk(&self, mint: &str) -> Result<TokenRisk> {
        self.core
            .get(&format!("/tokens/{}/risk", mint), &())
            .await
    }

    /// v0.8 — Filtered, sortable token directory (PRO+). Default `min_liq=2000`
    /// trims the long tail of phantom-MC tokens (low-liq pools producing absurd
    /// VWAP × supply products); set `Some(0.0)` to opt out. Computed filters
    /// (`min_volume_1h_usd`, `max_mev_share_pct`, `mc_change_1h_*`) over-fetch
    /// 3× from the DB and filter in app — pagination page size may be smaller
    /// than `limit` when those are set. Check `pagination.post_filtered` to
    /// detect.
    pub async fn list(&self, params: &TokensListParams) -> Result<TokensListResponse> {
        self.core.get("/tokens", params).await
    }
}
