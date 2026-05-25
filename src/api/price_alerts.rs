use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Price-drop / recovery alert rules CRUD (v1.9) — PRO 5 rules, ULTRA 20 rules.
///
/// Alerts fire when a token's market cap drops below the configured threshold
/// (and optionally again on recovery). Delivered via WebSocket channel
/// `price:alerts` and/or HMAC-signed webhook.
#[derive(Debug, Clone)]
pub struct PriceAlerts {
    pub(crate) core: Arc<HttpCore>,
}

impl PriceAlerts {
    /// List your price alert rules.
    pub async fn list(&self) -> Result<PriceAlertListResponse> {
        self.core.get("/price-alerts", &()).await
    }

    /// Create a price alert rule.
    /// Returns the rule plus a one-time `webhook_secret` — save it for HMAC verification.
    pub async fn create(
        &self,
        params: &PriceAlertCreateParams,
    ) -> Result<PriceAlertCreateResponse> {
        self.core.post_json("/price-alerts", params).await
    }

    /// Fetch a single alert by id.
    pub async fn get(&self, id: i64) -> Result<PriceAlertGetResponse> {
        self.core
            .get(&format!("/price-alerts/{}", id), &())
            .await
    }

    /// Update an alert (toggle `is_active`, change `delivery_mode`, etc).
    /// Thresholds (`drop_pct`, `recovery_pct`) are immutable after creation.
    pub async fn update(
        &self,
        id: i64,
        params: &PriceAlertUpdateParams,
    ) -> Result<PriceAlertUpdateResponse> {
        self.core
            .patch_json(&format!("/price-alerts/{}", id), params)
            .await
    }

    /// Delete an alert and its event history.
    pub async fn delete(&self, id: i64) -> Result<PriceAlertDeleteResponse> {
        self.core
            .delete(&format!("/price-alerts/{}", id))
            .await
    }

    /// Fired event history (30-day retention). Filter by `alert_id`, `event_type`, `since`.
    pub async fn events(
        &self,
        params: &PriceAlertEventsParams,
    ) -> Result<PriceAlertEventsResponse> {
        self.core.get("/price-alerts/events", params).await
    }
}
