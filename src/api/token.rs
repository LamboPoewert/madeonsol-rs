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
}
