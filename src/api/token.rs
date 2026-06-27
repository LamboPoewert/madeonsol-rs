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

    /// v0.15 — 1-minute OHLC candles for a token, aggregated from the trade
    /// firehose. Returns open/high/low/close, USD volume, trade count, and
    /// market cap per bar. ULTRA unlocks buy/sell volume split, net flow,
    /// liquidity, MC high/low, and MEV volume per candle.
    ///
    /// Use [`CandlesParams`] to pick the timeframe (`tf`), `limit`, and an
    /// optional `from`/`to` time window — unset params are omitted from the
    /// query string.
    pub async fn candles(&self, mint: &str, params: &CandlesParams) -> Result<CandlesResponse> {
        self.core
            .get(&format!("/tokens/{}/candles", mint), params)
            .await
    }

    /// v0.16 — Aggregated buy/sell flow for a token over a rolling window (PRO+).
    /// Returns unique wallet/buyer/seller counts, buy/sell counts and SOL volumes,
    /// `net_sol` (`buy_sol` − `sell_sol`), and `trades_per_wallet`.
    ///
    /// Use [`TokenFlowParams`] to pick the `window` (`"1h"` default or `"24h"`) —
    /// an unset param is omitted from the query string.
    pub async fn token_flow(
        &self,
        mint: &str,
        params: &TokenFlowParams,
    ) -> Result<TokenFlowResponse> {
        self.core
            .get(&format!("/tokens/{}/flow", mint), params)
            .await
    }

    /// v0.8 — Filtered, sortable token directory (PRO+). Default `min_liq=2000`
    /// trims the long tail of phantom-MC tokens (low-liq pools producing absurd
    /// VWAP × supply products); set `Some(0.0)` to opt out. Computed filters
    /// (`min_volume_1h_usd`, `max_mev_share_pct`, `mc_change_1h_*`) over-fetch
    /// 3× from the DB and filter in app — pagination page size may be smaller
    /// than `limit` when those are set. Check `pagination.post_filtered` to
    /// detect.
    ///
    /// `sort` accepts (among others) the momentum values `"mc_change_5m_desc"`,
    /// `"mc_change_1h_desc"`, `"volume_1h_desc"`, and `"trending"`.
    pub async fn list(&self, params: &TokensListParams) -> Result<TokensListResponse> {
        self.core.get("/tokens", params).await
    }

    /// v0.18 — Pre-bond pump.fun tokens near graduation, ranked by velocity
    /// (PRO/ULTRA). Surfaces tokens climbing their bonding curve, with
    /// `progress_pct`, `velocity_pct_per_min`, `eta_minutes`, and a `stalled`
    /// flag for momentum that has stopped.
    ///
    /// Use [`AlmostBondedParams`] to filter by progress band, velocity floor,
    /// age, deployer tier, authority-revoked status, and liquidity, and to pick
    /// the [`AlmostBondedSort`] order (`velocity_desc` default) — unset params
    /// are omitted from the query string.
    pub async fn almost_bonded(
        &self,
        params: &AlmostBondedParams,
    ) -> Result<AlmostBondedResponse> {
        self.core.get("/tokens/almost-bonded", params).await
    }
}
