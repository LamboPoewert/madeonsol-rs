use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// First-touch webhook subscriptions CRUD — ULTRA only.
///
/// Each subscription delivers `kol:first_touch` events that match the configured
/// filters as HMAC-signed POSTs to your `webhook_url`. Up to 10 active per ULTRA user.
/// Use `kol.first_touches()` for read-only queries on the same data.
#[derive(Debug, Clone)]
pub struct FirstTouchSubscriptions {
    pub(crate) core: Arc<HttpCore>,
}

impl FirstTouchSubscriptions {
    /// List your first-touch webhook subscriptions.
    pub async fn list(&self) -> Result<FirstTouchSubscriptionListResponse> {
        self.core.get("/kol/first-touches/subscriptions", &()).await
    }

    /// Create a first-touch webhook subscription.
    /// Returns the subscription plus a one-time `webhook_secret` — save it for HMAC verification.
    pub async fn create(
        &self,
        params: &FirstTouchSubscriptionCreateParams,
    ) -> Result<FirstTouchSubscriptionCreateResponse> {
        self.core
            .post_json("/kol/first-touches/subscriptions", params)
            .await
    }

    /// Fetch a single subscription by id.
    pub async fn get(&self, id: &str) -> Result<FirstTouchSubscriptionGetResponse> {
        self.core
            .get(&format!("/kol/first-touches/subscriptions/{}", id), &())
            .await
    }

    /// Update a subscription (toggle `is_active`, change filters, etc).
    pub async fn update(
        &self,
        id: &str,
        params: &FirstTouchSubscriptionUpdateParams,
    ) -> Result<FirstTouchSubscriptionUpdateResponse> {
        self.core
            .patch_json(&format!("/kol/first-touches/subscriptions/{}", id), params)
            .await
    }

    /// Delete a subscription.
    pub async fn delete(&self, id: &str) -> Result<FirstTouchSubscriptionDeleteResponse> {
        self.core
            .delete(&format!("/kol/first-touches/subscriptions/{}", id))
            .await
    }
}
