# madeonsol

[![Crates.io](https://img.shields.io/crates/v/madeonsol?style=flat-square)](https://crates.io/crates/madeonsol)
[![docs.rs](https://img.shields.io/docsrs/madeonsol?style=flat-square)](https://docs.rs/madeonsol)
[![Crates.io downloads](https://img.shields.io/crates/d/madeonsol?style=flat-square)](https://crates.io/crates/madeonsol)
[![GitHub stars](https://img.shields.io/github/stars/LamboPoewert/madeonsol-rs?style=flat-square&logo=github)](https://github.com/LamboPoewert/madeonsol-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

> ⭐ **[Star on GitHub](https://github.com/LamboPoewert/madeonsol-rs)** · 📂 **[Examples](./examples/)** · 📚 **[docs.rs](https://docs.rs/madeonsol)** · 🌐 **[API docs](https://madeonsol.com/api-docs)**

Official Rust SDK for the **[MadeOnSol](https://madeonsol.com) Solana API** — typed,
async, `tokio`-based, `rustls`-only.

> Real-time Solana trading intelligence: track 1,000+ KOL wallets with <3s latency,
> score 6,700+ Pump.fun deployers by reputation, detect multi-KOL coordination
> signals, and stream every DEX trade across 9+ programs.
>
> **Free tier: 200 requests/day at <https://madeonsol.com/pricing> — no credit card required.**

> **New in 0.8.0** *(2026-05-12)* — **Token directory + account inspection.**
> `client.token.list(&TokensListParams { min_liq: Some(10_000.0), min_volume_1h_usd: Some(5_000.0), max_mev_share_pct: Some(60.0), mc_change_1h_min_pct: Some(20.0), sort: Some("mc_desc".into()), ..Default::default() })` filters every active mint by MC band, liquidity floor, primary DEX,
> authority/safety flags, computed 1h volume, MEV-share ceiling, and MC-change deltas. Default `min_liq=2000` skips phantom-MC dust;
> pass `Some(0.0)` to opt out. `client.me.get()` — read your tier, daily/burst
> quota state, and per-feature usage in one call (no header parsing). Velocity / MEV-share fields added
> to every `TokenResponseBody`: `velocity["5m"|"15m"|"1h"|"2h"|"4h"]` with
> `mc_change_pct`, `volume_usd`, `mev_volume_pct`. `/token/{mint}` 400s now ship
> structured `code`, `reason`, `received_length`, `example`, and `docs`. Deprecated `avg_entry_mc_usd` / `entry_mc_samples` fully removed.

## Get an API key

1. Visit **<https://madeonsol.com/developer>**
2. Sign in with email or Solana wallet
3. Copy your `msk_…` key (free tier is unlocked instantly — 200 req/day, 10/min)

Paid tiers unlock higher rate limits, sub-hour windows, WebSocket streaming,
webhooks, and the all-DEX firehose:

| Tier | Price | Daily req | KOL trending sub-hour | Stream | Webhooks | DEX firehose |
|---|---|---|---|---|---|---|
| **Free** | $0 | 200 | — | — | — | — |
| **PRO** | $49/mo | 10,000 | ✅ | ✅ | 3 | — |
| **ULTRA** | $149/mo | 100,000 | ✅ | ✅ | 10 | ✅ |

Annual: PRO $490/yr, ULTRA $1,490/yr (2 months free).

## Install

```toml
[dependencies]
madeonsol = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.75+. Uses `reqwest` with `rustls-tls` (no OpenSSL dependency).

## Quick start

```rust
use madeonsol::{MadeOnSol, types::{KolFeedParams, KolAction}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free key — get one at https://madeonsol.com/pricing
    let client = MadeOnSol::new(std::env::var("MADEONSOL_API_KEY")?)?;

    let feed = client.kol.feed(&KolFeedParams {
        limit: Some(10),
        action: Some(KolAction::Buy),
        ..Default::default()
    }).await?;

    for trade in feed.trades {
        let mc = trade.market_cap_usd_at_trade
            .map(|m| format!(" @ MC ${:.0}", m))
            .unwrap_or_default();
        println!("{:?} bought {:?} for {} SOL{}",
                 trade.kol_name, trade.token_symbol, trade.sol_amount, mc);
    }
    Ok(())
}
```

Run the bundled examples:

```sh
export MADEONSOL_API_KEY=msk_...
cargo run --example kol_feed
cargo run --example deployer_alerts
```

## Namespaces

The `MadeOnSol` client exposes namespaced sub-clients:

| Namespace | Purpose |
|---|---|
| `client.kol` | KOL feed, leaderboard, coordination, PnL, trending tokens, alerts, compare, **first_touches** |
| `client.deployer` | Pump.fun deployer leaderboard, alerts, trajectory, bonded tokens |
| `client.alpha` | Alpha-wallet leaderboard, profiles, cap tables, buyer quality |
| `client.wallet_tracker` | Track arbitrary Solana wallets — watchlist CRUD, swap/transfer history |
| `client.coordination_alerts` | Push alerts on coordinated buying (PRO/ULTRA) |
| `client.first_touch_subscriptions` | Push alerts on first-KOL-touch events (ULTRA) |
| `client.tools` | Solana tool directory search |
| `client.stream` | Issue 24h WebSocket streaming tokens |
| `client.webhooks` | Webhook CRUD (PRO/ULTRA) |

Full reference: <https://docs.rs/madeonsol> · Interactive API docs: <https://madeonsol.com/api-docs>.

## Use cases

- **Copy-trading bot** — stream KOL buys via `client.kol.feed()` and mirror trades
- **DEX trade sniping** — subscribe to the all-DEX stream filtered by token / wallet
- **Deployer sniper** — monitor `client.deployer.alerts()` for elite-tier launches
- **Coordination detector** — flag tokens with `client.kol.coordination()` or push alerts
- **Scout signal** — track first-KOL-touch events filtered to S/A-tier scouts via `client.kol.first_touches()` (backtested: ~50% swarm rate vs 14% baseline)
- **Analytics dashboard** — combine leaderboard, PnL, and tool data
- **Telegram/Discord bot** — pipe alerts via webhooks into chat
- **Portfolio tracker** — use `client.kol.wallet()` to follow specific KOL positions

## Error handling

All methods return `Result<T, madeonsol::Error>`. The `Error::Api` variant exposes
HTTP status, server message, and the raw JSON body:

```rust
use madeonsol::{MadeOnSol, Error};

# async fn run(client: MadeOnSol) -> Result<(), Error> {
match client.kol.token("invalid-mint").await {
    Ok(activity) => println!("{:?}", activity),
    Err(Error::Api { status, message, .. }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(other) => return Err(other),
}
# Ok(())
# }
```

`Error::MissingApiKey` is returned by `MadeOnSol::new` if the key is empty or
doesn't start with `msk_` — the error message and a stderr hint both link to
<https://madeonsol.com/pricing>.

## First-touch signal *(new in 0.4)*

Every "first KOL buy on a token mint" event — when a tracked KOL is the first of the cohort to touch a token. Filterable by **scout tier** (S/A/B/C from `mv_kol_scout_score`), KOL winrate, token age, mint suffix.

**Backtest:** S-tier scouts attract ≥3 follow-on KOLs within 4h ~50% of the time vs ~14% baseline (38d / 491k buys / 72,549 events). Public leaderboard at <https://madeonsol.com/kol/scouts>.

```rust
use madeonsol::{FirstTouchPreset, FirstTouchesParams, ScoutTier};

let res = client
    .kol
    .first_touches(&FirstTouchesParams {
        preset: Some(FirstTouchPreset::Scout),
        min_scout_tier: Some(ScoutTier::S),
        limit: Some(20),
        ..Default::default()
    })
    .await?;

for e in res.events {
    println!(
        "{} scouted {} (scout_score={:?}%)",
        e.first_kol.name.unwrap_or_default(),
        e.token_symbol.unwrap_or_default(),
        e.first_kol.scout_score
    );
}
```

**Webhook subscriptions** (Ultra, up to 10 active per user) — push delivery, HMAC-SHA256 signed:

```rust
use madeonsol::{FirstTouchSubscriptionCreateParams, FirstTouchSubscriptionFilters, ScoutTier, CoordinationDeliveryMode};

let res = client
    .first_touch_subscriptions
    .create(&FirstTouchSubscriptionCreateParams {
        name: Some("S-tier scouts on pump tokens".into()),
        filters: Some(FirstTouchSubscriptionFilters {
            min_scout_tier: Some(ScoutTier::S),
            mint_suffix: Some("pump".into()),
            ..Default::default()
        }),
        delivery_mode: Some(CoordinationDeliveryMode::Webhook),
        webhook_url: Some("https://you.com/hooks/scout".into()),
    })
    .await?;
// store res.webhook_secret — shown ONCE
```

> **Don't poll — push.** Median lead time before the second KOL is 12 seconds. WebSocket channel: `kol:first_touches` (PRO+).

## WebSocket streams (PRO/ULTRA)

This crate does **not** ship a WebSocket client — `client.stream.get_token()`
returns the URL + token, and you connect with any WS library
(`tokio-tungstenite` recommended):

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
let token = client.stream.get_token().await?;
let ws_url = format!("{}?token={}", token.ws_url, token.token);
// then: tokio_tungstenite::connect_async(&ws_url).await
# Ok(())
# }
```

The DEX firehose URL (`token.dex_ws_url`) is only present for ULTRA subscribers.
See <https://madeonsol.com/api-docs> for the full subscribe/unsubscribe protocol.

## Also available

| Platform | Package |
|---|---|
| TypeScript / Node | [`madeonsol`](https://www.npmjs.com/package/madeonsol) on npm |
| Python (LangChain, CrewAI) | [`madeonsol-x402`](https://pypi.org/project/madeonsol-x402/) on PyPI |
| MCP Server (Claude, Cursor) | [`mcp-server-madeonsol`](https://www.npmjs.com/package/mcp-server-madeonsol) |
| ElizaOS | [`@madeonsol/plugin-madeonsol`](https://www.npmjs.com/package/@madeonsol/plugin-madeonsol) |
| Solana Agent Kit | [`solana-agent-kit-plugin-madeonsol`](https://www.npmjs.com/package/solana-agent-kit-plugin-madeonsol) |

## Links

- API docs: <https://madeonsol.com/api-docs> (interactive try-it-now)
- OpenAPI 3.1 spec: <https://madeonsol.com/api/v1/openapi.json>
- Postman collection: <https://madeonsol.com/api/v1/postman.json>
- Pricing & free key: <https://madeonsol.com/pricing>
- Issues: <https://github.com/LamboPoewert/madeonsol-rs/issues>

## License

MIT © [MadeOnSol](https://madeonsol.com)
