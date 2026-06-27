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
> signals, push every pump.fun graduation the second it bonds, and stream every
> DEX trade across 9+ programs.
>
> **Free tier: 200 requests/day at <https://madeonsol.com/pricing> — no credit card required.**

> **This is the keyed REST SDK** — authenticate with an API key (`msk_…`). It covers the full endpoint surface (KOL intelligence, deployer intel, token risk/buyer-quality, Signal Scorecard, wallet PnL, DEX firehose). Want **x402 pay-per-call** instead — no signup, your agent's wallet pays per request in USDC? Use the TypeScript [`madeonsol-x402`](https://www.npmjs.com/package/madeonsol-x402) or Python [`madeonsol-x402`](https://pypi.org/project/madeonsol-x402/) clients.

> **New in 0.18.0** — **Almost-bonded tokens + trending sorts.** `client.token.almost_bonded(&params)` (`GET /tokens/almost-bonded`, PRO/ULTRA) returns pre-bond pump.fun tokens near graduation, ranked by velocity: each `AlmostBondedToken` carries `progress_pct`, `velocity_pct_per_min`, `eta_minutes`, a `stalled` flag, `real_sol_reserves`, `market_cap_usd`, `liquidity_usd`, `authorities_revoked`, `deployer_tier`, and `age_minutes`. `AlmostBondedParams` filters by `min_progress`/`max_progress`, `min_velocity_pct_per_min`, `max_age_minutes`, `deployer_tier`, `authority_revoked`, and `min_liq`, and picks the `AlmostBondedSort` order (`VelocityDesc` default, `ProgressDesc`, `EtaAsc`). New types: `AlmostBondedParams`, `AlmostBondedSort`, `AlmostBondedToken`, `AlmostBondedResponse`. `client.token.list(&params)` also accepts four new momentum `sort` values: `"mc_change_5m_desc"`, `"mc_change_1h_desc"`, `"volume_1h_desc"`, and `"trending"`.

> **New in 0.17.0** — **Token flow + deployer SOL balance.** `client.token.token_flow(mint, &params)` (`GET /tokens/{mint}/flow`, PRO+) returns aggregated buy/sell flow for a token over a rolling window: `unique_wallets`/`unique_buyers`/`unique_sellers`, `buy_count`/`sell_count`/`total_trades`, `buy_sol`/`sell_sol`/`net_sol` (buy − sell), and `trades_per_wallet`, plus the window `from` timestamp. `TokenFlowParams { window: Some("24h".into()) }` selects the window (`"1h"` default or `"24h"`). New types: `TokenFlowParams`, `TokenFlowResponse`. `DeployerAlert` also gains `deployer_sol_balance: Option<f64>` — the deployer wallet's SOL balance at alert time.

> **New in 0.16.0** — **Signal Scorecard.** New `client.signals` namespace. `client.signals.catalog()` returns the discovery index — every available signal with its `methodology` and a `performance_endpoint` (`SignalsCatalog`, `SignalCatalogEntry`). `client.signals.performance(name, &params)` returns a named signal's out-of-sample, machine-readable reliability — per-bucket `hit_rate` vs `base_rate`, `lift`, and `sample_n`, plus the test window and `methodology` (`SignalPerformance`, `SignalBucket`). Pass `SignalPerformanceParams { history: Some(true) }` to append the per-day drift series (`SignalHistoryEntry`). Valid signal names: `dump_cluster_count`, `runner_rate`, `recycled_early_buyer_count`, `coordination_count`. Open to any authenticated tier. New types: `SignalPerformanceParams`, `SignalPerformance`, `SignalBucket`, `SignalHistoryEntry`, `SignalsCatalog`, `SignalCatalogEntry`.

> **New in 0.15.0** — **OHLC candles.** `client.token.candles(mint, &params)` returns 1-minute OHLC candles aggregated from the trade firehose (PRO/ULTRA): per-bar `open`/`high`/`low`/`close`, `volume_usd`, `trades`, and `market_cap_usd`. ULTRA unlocks buy/sell volume split (`buy_volume_usd`, `sell_volume_usd`, `net_volume_usd`), open/close liquidity, MC high/low, buy/sell counts, and MEV volume per candle. `CandlesParams` selects `tf`, `limit`, and an optional `from`/`to` window. New types: `CandlesParams`, `Candle`, `CandlesResponse`.

> **New in 0.14.0** — **Token risk score.** `client.token.risk(mint)` returns a transparent 0–100 rug-risk / safety score (PRO/ULTRA, higher = riskier): an overall `risk_score` + `RiskBand` (`safe`/`caution`/`danger`), a per-factor `Vec<RiskFactor>` breakdown (each with `status`, `points`, and a human-readable `detail`), and the raw `RiskInputs` every factor was derived from (mint/freeze authority revocation, liquidity, transfer fee, launch cohort, deployer reputation, blacklist, …). New types: `TokenRisk`, `RiskFactor`, `RiskInputs`, `RiskBand`, `RiskFactorStatus`.

> **New in 0.13.0** — **Launch cohort, liquidity/MC ratio, deployer-tier filter, and KOL leaderboard timing.** `TokenResponseBody` gains `liquidity_to_mc_ratio`, `launch_cohort_sol`, and `launch_cohort_size`. `TokensListParams` gains `min_liq_mc_ratio`, `max_liq_mc_ratio`, and `deployer_tier` filters. `TokenSummary` (tokens list items) gains `liquidity_to_mc_ratio` and `deployer_tier`. `KolLeaderboardEntry` gains `median_hold_minutes_30d` and `percentile_early_entry_30d`.

> **New in 0.12.1** — **Deployer runner-rate fields.** `SniperDeploy`, `DeployerSummary`, `DeployerProfile`, and `DeployerLeaderboardEntry` now carry `runner_rate` (fraction of the deployer's labeled tokens that ran — peak ≥60min after deploy — vs dumped) and `labeled_tokens` (confidence denominator; gate on ≥3).

> **New in 0.12.0** *(2026-06-07)* — **Graduation events + dump-cluster detection.** `GraduationEvent` — typed payload for the `token:graduations` WebSocket channel: every pump.fun bond in real time (tracked deployer or not) with deployer tier, time-to-bond, and MC at bond. `AlphaBuyerQualityBreakdown` adds `dump_cluster_count` (out-of-sample: 3+ such wallets in the first-20 → 94% dump vs 61% base) and `recycled_early_buyer_count`. DEX firehose: replay buffer deepened to ~5 min; mint-scoped subs get in-band `dex:graduations` frames.

> **New in 0.10.0** *(2026-05-25)* — **Price alerts, scout leaderboard, KOL consensus, peak history, coordination history, wallet derived stats, trajectory snapshots.**
> `client.price_alerts` — full CRUD for MC-drop / recovery alert rules (PRO/ULTRA). `client.kol.scout_leaderboard()` — ranked scouts by swarm attraction rate. `client.token.kol_consensus(mint)` — per-token KOL buyer/seller breakdown. `client.token.peak_history(mint)` — ATH, decline from peak, MC snapshots post-bond. `client.kol.coordination_history()` — past coordination fires. `client.deployer.trajectory(wallet, params)` now accepts `include: Some("daily_snapshots")` for 90d snapshots. `WalletStatsResponse.derived` — win rate, ROI, best/worst trade, biggest miss, AI verdict.

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
madeonsol = "0.18"
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
| `client.kol` | KOL feed, leaderboard, coordination, PnL, trending tokens, alerts, compare, **first_touches**, **scout_leaderboard**, **coordination_history** |
| `client.deployer` | Pump.fun deployer leaderboard, alerts, trajectory (+ daily snapshots), bonded tokens |
| `client.alpha` | Alpha-wallet leaderboard, profiles, cap tables, buyer quality |
| `client.token` | Per-mint snapshot, batch lookup, buyer quality, **kol_consensus**, **peak_history**, **risk**, **candles**, **token_flow**, **almost_bonded**, directory list |
| `client.wallet_tracker` | Track arbitrary Solana wallets — watchlist CRUD, swap/transfer history |
| `client.wallet` | Universal wallet endpoints — stats + cross-product flags + derived analytics, FIFO PnL, open positions, paginated trades (PRO+) |
| `client.coordination_alerts` | Push alerts on coordinated buying (PRO/ULTRA) |
| `client.first_touch_subscriptions` | Push alerts on first-KOL-touch events (ULTRA) |
| `client.price_alerts` *(new 0.10)* | MC-drop / recovery price alert rules CRUD + event history (PRO/ULTRA) |
| `client.signals` *(new 0.16)* | **Signal Scorecard** — out-of-sample, machine-readable signal reliability (`performance`) + discovery catalog |
| `client.sniper` *(new 0.11)* | **Deshred** pre-confirm pump.fun deploy feed (~500ms head start) + custom deployer watchlist (PRO/ULTRA) |
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

## Universal wallet endpoints *(new in 0.9)*

Per-wallet profile data for **any** Solana wallet — not just curated KOLs. FIFO cost-basis PnL over the last 90 days, cached server-side with dynamic TTL. Cache hits don't count against your daily quota. PRO+.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::WalletTradesParams;

// 1. Profile any wallet — works on KOLs, alpha traders, deployers, randoms.
let stats = client.wallet.stats("ASVzakePP6GNg9r95d4LPZHJDMXun6L6E4um4pu5ybJk").await?;
if let Some(s) = stats.stats {
    println!("{}: {} trades, {} unique tokens", stats.address, s.total_trades, s.unique_tokens);
}
println!("KOL: {} · alpha: {} · deployer: {}",
    stats.flags.is_kol, stats.flags.is_alpha_tracked, stats.flags.is_deployer);

// 2. Full FIFO PnL — realized + unrealized SOL, profit factor, drawdown,
//    daily curve, closed/open positions.
let pnl = client.wallet.pnl(&stats.address).await?;
println!("Realized: {:+.2} SOL · Unrealized: {:+.2} SOL", pnl.summary.realized_sol, pnl.summary.unrealized_sol);
if let Some(pf) = pnl.summary.profit_factor {
    println!("Win rate: {:.0}% · Profit factor: {:.2}", pnl.summary.win_rate.unwrap_or(0.0) * 100.0, pf);
}
for c in pnl.closed_positions.iter().take(5) {
    println!("  {}…  {:+.2} SOL ({:?}% ROI, {} min hold)",
        &c.token_mint[..8], c.pnl_sol, c.roi_pct, c.hold_minutes.unwrap_or(0));
}

// 3. Paginated raw trades — keep paging with next_cursor.
let mut params = WalletTradesParams { limit: Some(200), ..Default::default() };
loop {
    let page = client.wallet.trades(&stats.address, &params).await?;
    for t in &page.trades { /* … */ }
    if !page.has_more { break; }
    params.cursor = page.next_cursor;
}
# Ok(())
# }
```

**Cost-basis honesty.** Observable only inside the 90-day window. Overflow sells (no matching buy in window) are silently discarded rather than fabricated. `notes.cost_basis_observable_from` makes the cutoff visible.

## Deshred sniper alerts *(new in 0.11)*

The fastest path to a new pump.fun launch. Deploys are reconstructed from shred-level (**deshred**) data and surface **~500ms before the chain confirms them**. **PRO** sees elite + good deployers; **ULTRA** sees every tier and can keep a custom deployer watchlist. For live push use the `sniper:deploy` webhook, the `sniper:deploys` WebSocket channel, or `/alert sniper` in Telegram — these methods are for catch-up, backtesting, and watchlist management.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{SniperRecentParams, SniperWatchlistAddParams};

// Deshred deploy feed — PRO: elite/good · ULTRA: all tiers
let feed = client.sniper.recent(&SniperRecentParams { limit: Some(50), ..Default::default() }).await?;
for d in &feed.deploys {
    println!("{} by {} (tier {:?})", d.symbol.as_deref().unwrap_or("?"), d.deployer_wallet, d.deployer_tier);
}

// Custom watchlist (ULTRA, max 50) — get deploys from only the deployers you track, any tier
client.sniper.add_to_watchlist(&SniperWatchlistAddParams {
    wallets: Some(vec!["7dEx...4pQ8".into(), "9aBc...2zZ1".into()]),
    label: Some("alpha devs".into()),
    ..Default::default()
}).await?;
let tracked = client.sniper.recent(&SniperRecentParams { watchlist: Some(true), ..Default::default() }).await?;
println!("{} deploys from watchlisted deployers", tracked.count);
# Ok(())
# }
```

## Price alerts *(new in 0.10)*

Get notified when a token's market cap drops below a threshold (and optionally on recovery). PRO: 5 rules, ULTRA: 20 rules. Delivered via WebSocket channel `price:alerts` and/or HMAC-signed webhook.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{PriceAlertCreateParams, PriceAlertDeliveryMode, PriceAlertEventsParams};

// Create an alert: fire when MC drops 30%, then again on 50% recovery.
let res = client.price_alerts.create(&PriceAlertCreateParams {
    token_mint: "So11111111111111111111111111111111111111112".into(),
    drop_pct: 30.0,
    recovery_pct: Some(50.0),
    name: Some("SOL 30% dip".into()),
    delivery_mode: Some(PriceAlertDeliveryMode::Webhook),
    webhook_url: Some("https://you.com/hooks/price".into()),
}).await?;
// store res.webhook_secret — shown ONCE
println!("Alert {} created, status: {:?}", res.alert.id, res.alert.status);

// List active alerts
let alerts = client.price_alerts.list().await?;
for a in alerts.alerts {
    println!("{}: {} drop={}% status={:?}", a.id, a.token_mint, a.drop_pct, a.status);
}

// Check fired events
let events = client.price_alerts.events(&PriceAlertEventsParams {
    limit: Some(20),
    ..Default::default()
}).await?;
for e in events.events {
    println!("{} {} at MC ${:.0}", e.event_type, e.token_mint, e.current_mc_usd);
}
# Ok(())
# }
```

## New in 0.10: scout leaderboard, KOL consensus, peak history

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::{ScoutLeaderboardParams, ScoutTier, ScoutLeaderboardSort};

// Top scouts by swarm attraction rate
let scouts = client.kol.scout_leaderboard(&ScoutLeaderboardParams {
    limit: Some(10),
    scout_tier: Some(ScoutTier::S),
    sort: Some(ScoutLeaderboardSort::Swarm3PlusPct),
}).await?;
println!("{}", scouts);

// KOL consensus on a token
let consensus = client.token.kol_consensus("So11111111111111111111111111111111111111112").await?;
println!("{} buyers, {} sellers, exit rate {:?}%",
    consensus.total_kol_buyers, consensus.total_kol_sellers, consensus.kol_exit_rate);

// Peak MC history
let peak = client.token.peak_history("So11111111111111111111111111111111111111112").await?;
println!("ATH: {:?}, decline: {:?}%", peak.peak_mc_usd, peak.decline_from_peak_pct);

// 1-minute OHLC candles (PRO/ULTRA)
use madeonsol::types::CandlesParams;
let candles = client.token.candles(
    "So11111111111111111111111111111111111111112",
    &CandlesParams { tf: Some("1m".into()), limit: Some(60), ..Default::default() },
).await?;
for c in &candles.candles {
    println!("{} O:{} H:{} L:{} C:{} vol:${}", c.t, c.open, c.high, c.low, c.close, c.volume_usd);
}

// Aggregated buy/sell flow over a window (PRO+)
use madeonsol::types::TokenFlowParams;
let flow = client.token.token_flow(
    "So11111111111111111111111111111111111111112",
    &TokenFlowParams { window: Some("24h".into()) },
).await?;
println!("{} wallets · net {} SOL", flow.unique_wallets, flow.net_sol);
# Ok(())
# }
```

## New in 0.16: Signal Scorecard

Out-of-sample, machine-readable reliability for each enrichment signal, so bots
can weight them programmatically instead of asking. Open to any authenticated tier.

```rust
# async fn run(client: madeonsol::MadeOnSol) -> Result<(), Box<dyn std::error::Error>> {
use madeonsol::types::SignalPerformanceParams;

// Discover the available signals + how to fetch each one's efficacy
let catalog = client.signals.catalog().await?;
for s in &catalog.signals {
    println!("{} → {}", s.name, s.performance_endpoint);
}

// Live, out-of-sample reliability for a named signal
let perf = client.signals.performance(
    "coordination_count",
    &SignalPerformanceParams { history: Some(false) },
).await?;
println!("methodology: {:?}", perf.methodology);
for b in &perf.buckets {
    println!("{}: hit {:?} vs base {:?} (lift {:?}, n={})",
        b.bucket, b.hit_rate, b.base_rate, b.lift, b.sample_n);
}
# Ok(())
# }
```

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
| MCP Server (Claude, Cursor) | [`mcp-server-madeonsol`](https://www.npmjs.com/package/mcp-server-madeonsol) · [Smithery](https://smithery.ai/servers/madeonsol/solana-kol-intelligence) · [Glama](https://glama.ai/mcp/servers/LamboPoewert/mcp-server-madeonsol) |
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
