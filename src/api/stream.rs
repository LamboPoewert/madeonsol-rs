use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// WebSocket streaming token issuance.
#[derive(Debug, Clone)]
pub struct Stream {
    pub(crate) core: Arc<HttpCore>,
}

impl Stream {
    /// Generate a 24-hour WebSocket streaming token.
    ///
    /// PRO/ULTRA: returns `ws_url` for KOL/deployer event streaming.
    /// ULTRA: also returns `dex_ws_url` for the all-DEX trade firehose.
    ///
    /// Connect by appending `?token=<token>` to the returned URL.
    pub async fn get_token(&self) -> Result<StreamToken> {
        self.core.post_empty("/stream/token").await
    }

    /// v0.19 — List your live WebSocket sessions (PRO/ULTRA).
    ///
    /// Each entry describes a currently-connected `ws-streaming` or `dex-stream`
    /// socket — its subscribed `channels`, `tier`, `connected_at`, `remote_ip`,
    /// and `messages_sent` so far. Handy for auditing connection-limit usage or
    /// spotting a ghost socket that's still holding a slot.
    pub async fn sessions(&self) -> Result<StreamSessionsResponse> {
        self.core.get("/stream/sessions", &()).await
    }

    /// v0.19 — Force-disconnect one of your live WebSocket sessions by id
    /// (PRO/ULTRA), freeing its connection slot. Returns `{ evicted: true, id }`.
    ///
    /// Pass the `id` from a [`StreamSession`](crate::types::StreamSession).
    /// Returns an `Error::Api` `404` if no live session has that id, or `400`
    /// if `id` isn't a positive integer.
    pub async fn kill_session(&self, id: &str) -> Result<StreamSessionEvicted> {
        self.core
            .delete(&format!("/stream/sessions/{}", id))
            .await
    }
}
