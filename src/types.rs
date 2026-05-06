//! Request and response types for every endpoint in the MadeOnSol API.
//!
//! Field names mirror the JSON wire format exactly so you can paste API examples
//! straight from the docs at <https://madeonsol.com/api-docs>.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Shared enums ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum KolAction {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeaderboardPeriod {
    #[serde(rename = "today")]
    Today,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "90d")]
    D90,
    #[serde(rename = "180d")]
    D180,
}

impl LeaderboardPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::D7 => "7d",
            Self::D30 => "30d",
            Self::D90 => "90d",
            Self::D180 => "180d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoordinationPeriod {
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "6h")]
    H6,
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
}

impl CoordinationPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "1h",
            Self::H6 => "6h",
            Self::H24 => "24h",
            Self::D7 => "7d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KolStrategy {
    Scalper,
    DayTrader,
    SwingTrader,
    Hodler,
    Mixed,
}

impl KolStrategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Scalper => "scalper",
            Self::DayTrader => "day_trader",
            Self::SwingTrader => "swing_trader",
            Self::Hodler => "hodler",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KolLeaderboardSort {
    Pnl,
    Winrate,
    Volume,
    AvgRoi,
    ProfitFactor,
    EarlyEntryPct,
    Consistency,
}

impl KolLeaderboardSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pnl => "pnl",
            Self::Winrate => "winrate",
            Self::Volume => "volume",
            Self::AvgRoi => "avg_roi",
            Self::ProfitFactor => "profit_factor",
            Self::EarlyEntryPct => "early_entry_pct",
            Self::Consistency => "consistency",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeployerTier {
    Elite,
    Good,
    Moderate,
    Rising,
    Cold,
    #[serde(rename = "unranked")]
    Unranked,
}

impl DeployerTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Elite => "elite",
            Self::Good => "good",
            Self::Moderate => "moderate",
            Self::Rising => "rising",
            Self::Cold => "cold",
            Self::Unranked => "unranked",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeployerSortField {
    BondingRate,
    RecentBondRate,
    TotalBonded,
    LastDeployAt,
}

impl DeployerSortField {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BondingRate => "bonding_rate",
            Self::RecentBondRate => "recent_bond_rate",
            Self::TotalBonded => "total_bonded",
            Self::LastDeployAt => "last_deploy_at",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertPeriod {
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "all")]
    All,
}

impl AlertPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::D7 => "7d",
            Self::D30 => "30d",
            Self::All => "all",
        }
    }
}

pub type BestTokensPeriod = AlertPeriod;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletTrackerSummaryPeriod {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
}

impl WalletTrackerSummaryPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H24 => "24h",
            Self::D7 => "7d",
            Self::D30 => "30d",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WalletTrackerEventType {
    Swap,
    Transfer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WalletTrackerAction {
    Buy,
    Sell,
    TransferIn,
    TransferOut,
}

impl WalletTrackerAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
            Self::TransferIn => "transfer_in",
            Self::TransferOut => "transfer_out",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationDeliveryMode {
    Websocket,
    Webhook,
    Both,
}

impl CoordinationDeliveryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Websocket => "websocket",
            Self::Webhook => "webhook",
            Self::Both => "both",
        }
    }
}

// ─── KOL ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolFeedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor — return trades strictly older than this ISO 8601 timestamp.
    /// Pass `next_before` from the previous response for polling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<KolAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sol: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_age_max_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_sells: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<LeaderboardPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<KolLeaderboardSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolWalletParams {
    /// Comma-separated extras: "pnl_by_token", "recent_winners", "recent_losers".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolCoordinationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CoordinationPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_avg_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_strategies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTradeDeployer {
    pub wallet: String,
    pub tier: String,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTrade {
    pub tx_signature: String,
    pub wallet_address: String,
    #[serde(default)]
    pub kol_name: Option<String>,
    #[serde(default)]
    pub kol_twitter: Option<String>,
    pub action: KolAction,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub sol_amount: f64,
    pub token_amount: f64,
    /// Token market cap in USD at the moment of trade (real-time, sourced
    /// from our in-memory price tracker — not Dexscreener spot, which lags).
    #[serde(default)]
    pub market_cap_usd_at_trade: Option<f64>,
    /// Token price in USD at the moment of trade.
    #[serde(default)]
    pub price_usd_at_trade: Option<f64>,
    pub traded_at: String,
    #[serde(default)]
    pub kol_strategy_tag: Option<String>,
    #[serde(default)]
    pub kol_auto_strategy_tag: Option<String>,
    #[serde(default)]
    pub kol_winrate_7d: Option<f64>,
    #[serde(default)]
    pub kol_winrate_30d: Option<f64>,
    #[serde(default)]
    pub kol_early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub kol_is_heating_up: Option<bool>,
    #[serde(default)]
    pub kol_percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub kol_percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub token_age_minutes: Option<f64>,
    #[serde(default)]
    pub deployer: Option<KolTradeDeployer>,
    #[serde(default)]
    pub deployer_tier: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolFeedResponse {
    pub trades: Vec<KolTrade>,
    pub count: u32,
    #[serde(default)]
    pub data_age_seconds: Option<u64>,
    /// Cursor for the next page — pass as `before` to fetch older trades.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardEntry {
    #[serde(default)]
    pub name: Option<String>,
    pub wallet: String,
    #[serde(default)]
    pub strategy_tag: Option<String>,
    #[serde(default)]
    pub auto_strategy_tag: Option<String>,
    pub pnl: f64,
    pub buy_count: u64,
    pub sell_count: u64,
    pub volume: f64,
    #[serde(default)]
    pub win_rate: Option<f64>,
    #[serde(default)]
    pub avg_roi: Option<f64>,
    #[serde(default)]
    pub profit_factor: Option<f64>,
    #[serde(default)]
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub consistency_7d: Option<f64>,
    #[serde(default)]
    pub is_heating_up: Option<bool>,
    #[serde(default)]
    pub is_cold: Option<bool>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_30d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_30d: Option<f64>,
    /// v0.6 (2026-05-06) — average market cap (USD) at the moment of each buy in the period.
    /// Null when no buys had a tracked MC stamp.
    #[serde(default)]
    pub avg_entry_mc_usd: Option<f64>,
    /// v0.6 — buys whose `market_cap_usd_at_trade` was non-null and counted toward
    /// `avg_entry_mc_usd`. Use to gauge confidence.
    #[serde(default)]
    pub entry_mc_samples: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardResponse {
    pub leaderboard: Vec<KolLeaderboardEntry>,
    pub period: String,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlByToken {
    pub mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub realized_pnl_usd: f64,
    pub buy_count: u32,
    pub sell_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolWalletProfile {
    pub wallet: String,
    pub kol_name: Option<String>,
    pub kol_twitter: Option<String>,
    pub total_pnl_usd: f64,
    pub win_rate: f64,
    pub trade_count: u32,
    #[serde(default)]
    pub pnl_by_token: Option<Vec<KolPnlByToken>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationKol {
    pub name: String,
    pub wallet: String,
    #[serde(default)]
    pub buy_sol: Option<f64>,
    #[serde(default)]
    pub sell_sol: Option<f64>,
    #[serde(default)]
    pub exited: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatedToken {
    pub token_mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub kol_count: u32,
    pub total_buys: u64,
    pub total_sells: u64,
    pub net_sol_flow: f64,
    pub signal: String,
    pub avg_winrate_7d: Option<f64>,
    pub entry_rank_avg: Option<f64>,
    pub unique_strategies: u32,
    pub strategies: Vec<String>,
    pub first_buy_at: String,
    pub last_buy_at: String,
    pub time_to_consensus_sec: i64,
    #[serde(default)]
    pub peak_window_start: Option<String>,
    #[serde(default)]
    pub peak_window_end: Option<String>,
    #[serde(default)]
    pub peak_kols: Option<u32>,
    #[serde(default)]
    pub peak_buys: Option<u64>,
    #[serde(default)]
    pub exited_count: Option<u32>,
    #[serde(default)]
    pub holders_count: Option<u32>,
    #[serde(default)]
    pub coordination_score: Option<u32>,
    /// v0.6 (2026-05-06) — market cap (USD) stamped on the cluster's chronologically-first KOL buy.
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    /// v0.6 — current market cap (USD), from `token_prices`.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// v0.6 — current last-trade price (USD).
    #[serde(default)]
    pub last_price_usd: Option<f64>,
    #[serde(default)]
    pub kols: Option<Vec<CoordinationKol>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCoordinationResponse {
    pub coordination: Vec<CoordinatedToken>,
    #[serde(default)]
    pub score_version: Option<String>,
    #[serde(default)]
    pub window_minutes: Option<u32>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub min_kols: Option<u32>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTokenActivity {
    pub mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub kol_buyers: Vec<String>,
    pub kol_sellers: Vec<String>,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_sol_volume: f64,
    pub recent_trades: Vec<KolTrade>,
}

// ─── KOL pairs / timing / hot-tokens / pnl / trending ───────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolPairsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_shared: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPairMember {
    pub name: String,
    #[serde(default)]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPair {
    pub kol_a: KolPairMember,
    pub kol_b: KolPairMember,
    pub shared_token_count: u32,
    #[serde(default)]
    pub agreement_rate: Option<f64>,
    #[serde(default)]
    pub shared_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPairsResponse {
    pub pairs: Vec<KolPair>,
    pub period: String,
    pub min_shared: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolTimingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTimingProfile {
    pub tokens_traded: u32,
    pub positions_closed: u32,
    pub avg_hold_minutes: Option<f64>,
    #[serde(default)]
    pub median_hold_minutes: Option<f64>,
    #[serde(default)]
    pub pct_closed_1h: Option<f64>,
    #[serde(default)]
    pub pct_closed_6h: Option<f64>,
    #[serde(default)]
    pub pct_closed_24h: Option<f64>,
    #[serde(default)]
    pub avg_buy_size_sol: Option<f64>,
    #[serde(default)]
    pub avg_sell_size_sol: Option<f64>,
    #[serde(default)]
    pub most_active_hours: Option<Vec<u32>>,
    #[serde(default)]
    pub hour_distribution: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTimingResponse {
    pub kol: KolPairMember,
    pub timing: KolTimingProfile,
    pub period: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolHotTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_avg_winrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_strategies: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HotToken {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub kols_total: u32,
    pub kols_recent: u32,
    pub acceleration: f64,
    pub total_buy_sol: f64,
    pub total_sell_sol: f64,
    pub net_flow: f64,
    pub first_kol_buy_age_minutes: Option<f64>,
    #[serde(default)]
    pub kols: Option<Vec<KolPairMember>>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    #[serde(default)]
    pub first_kol_buy_at: Option<String>,
    #[serde(default)]
    pub last_kol_buy_at: Option<String>,
    #[serde(default)]
    pub time_to_consensus_sec: Option<i64>,
    #[serde(default)]
    pub avg_winrate_7d: Option<f64>,
    #[serde(default)]
    pub entry_rank_avg: Option<f64>,
    #[serde(default)]
    pub unique_strategies: Option<u32>,
    #[serde(default)]
    pub strategies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolHotTokensResponse {
    pub hot_tokens: Vec<HotToken>,
    pub period: String,
    pub min_kols: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolPnlParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlSummary {
    pub realized_pnl_sol: f64,
    pub total_volume_sol: f64,
    pub tokens_traded: u32,
    pub closed_positions: u32,
    pub open_positions: u32,
    pub win_count: u32,
    pub loss_count: u32,
    pub win_rate: Option<f64>,
    pub profit_factor: Option<f64>,
    pub best_trade_pnl_sol: f64,
    pub worst_trade_pnl_sol: f64,
    pub avg_roi_pct: Option<f64>,
    pub avg_hold_minutes: f64,
    pub median_hold_minutes: f64,
    pub max_drawdown_sol: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlCurvePoint {
    pub date: String,
    pub day_pnl: f64,
    pub cumulative_pnl: f64,
    pub trades: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolClosedPosition {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_count: u32,
    pub sell_count: u32,
    pub bought_sol: f64,
    pub sold_sol: f64,
    pub pnl_sol: f64,
    pub roi_pct: f64,
    pub hold_minutes: f64,
    pub result: String,
    pub first_trade: String,
    pub last_trade: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolOpenPosition {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_count: u32,
    pub bought_sol: f64,
    pub first_buy_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlIdentity {
    pub name: String,
    #[serde(default)]
    pub wallet: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub strategy_tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolPnlResponse {
    pub kol: KolPnlIdentity,
    pub summary: KolPnlSummary,
    #[serde(default)]
    pub pnl_curve: Option<Vec<KolPnlCurvePoint>>,
    #[serde(default)]
    pub closed_positions: Option<Vec<KolClosedPosition>>,
    #[serde(default)]
    pub open_positions: Option<Vec<KolOpenPosition>>,
    pub period: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolTrendingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrendingToken {
    pub token_mint: String,
    pub token_symbol: String,
    pub token_name: String,
    pub buy_volume_sol: f64,
    pub sell_volume_sol: f64,
    pub net_flow_sol: f64,
    pub buy_count: u32,
    pub sell_count: u32,
    pub kol_count: u32,
    pub latest_buy_age_minutes: Option<f64>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    #[serde(default)]
    pub first_buy_at: Option<String>,
    #[serde(default)]
    pub latest_buy_at: Option<String>,
    #[serde(default)]
    pub kols: Option<Vec<KolPairMember>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolTrendingResponse {
    pub trending: Vec<TrendingToken>,
    pub period: String,
    pub min_kols: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── KOL entry-order / compare / alerts ─────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolEntryOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolEntryOrderEntry {
    pub rank: u32,
    pub kol_name: Option<String>,
    pub kol_twitter: Option<String>,
    pub wallet: String,
    pub strategy_tag: Option<String>,
    pub auto_strategy_tag: Option<String>,
    pub winrate_7d: Option<f64>,
    pub winrate_30d: Option<f64>,
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    pub first_buy_at: String,
    pub seconds_after_first: f64,
    pub sol_amount: f64,
    pub token_amount: f64,
    #[serde(default)]
    pub tx_signature: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolEntryOrderResponse {
    pub token_mint: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub total_kol_buyers: u32,
    pub first_buy_at: String,
    pub last_buy_at: String,
    pub span_sec: i64,
    pub entries: Vec<KolEntryOrderEntry>,
}

#[derive(Debug, Clone)]
pub struct KolCompareParams {
    /// 2–5 Solana wallet addresses. Tier limit: BASIC=2, PRO=4, ULTRA=5.
    pub wallets: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareProfile {
    pub wallet: String,
    pub found: bool,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub strategy_tag: Option<String>,
    #[serde(default)]
    pub auto_strategy_tag: Option<String>,
    #[serde(default)]
    pub winrate_7d: Option<f64>,
    #[serde(default)]
    pub winrate_30d: Option<f64>,
    #[serde(default)]
    pub avg_roi_7d: Option<f64>,
    #[serde(default)]
    pub avg_roi_30d: Option<f64>,
    #[serde(default)]
    pub profit_factor_7d: Option<f64>,
    #[serde(default)]
    pub profit_factor_30d: Option<f64>,
    #[serde(default)]
    pub pnl_7d: Option<f64>,
    #[serde(default)]
    pub pnl_30d: Option<f64>,
    #[serde(default)]
    pub early_entry_pct_30d: Option<f64>,
    #[serde(default)]
    pub consistency_7d: Option<f64>,
    #[serde(default)]
    pub median_hold_minutes_30d: Option<f64>,
    #[serde(default)]
    pub closed_positions_7d: Option<u32>,
    #[serde(default)]
    pub closed_positions_30d: Option<u32>,
    #[serde(default)]
    pub is_heating_up: Option<bool>,
    #[serde(default)]
    pub is_cold: Option<bool>,
    #[serde(default)]
    pub percentile_pnl_7d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_7d: Option<f64>,
    #[serde(default)]
    pub percentile_pnl_30d: Option<f64>,
    #[serde(default)]
    pub percentile_winrate_30d: Option<f64>,
    #[serde(default)]
    pub percentile_early_entry_30d: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareOverlapToken {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub wallets: Vec<String>,
    pub buy_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolCompareResponse {
    pub profiles: Vec<KolCompareProfile>,
    #[serde(default)]
    pub overlap: Option<Vec<KolCompareOverlapToken>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KolAlertsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolAlert {
    #[serde(rename = "type")]
    pub alert_type: String,
    pub severity: String,
    pub detected_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolAlertsResponse {
    pub alerts: Vec<KolAlert>,
    pub count: u32,
    pub window: String,
    pub types: Vec<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Coordination alerts (v1.1) ─────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertRule {
    pub id: String,
    pub name: Option<String>,
    pub min_kols: u32,
    pub window_minutes: u32,
    pub min_score: u32,
    pub include_majors: bool,
    pub cooldown_min: u32,
    pub score_jump_break: u32,
    pub delivery_mode: CoordinationDeliveryMode,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationAlertCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_jump_break: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationAlertUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_majors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_jump_break: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertListResponse {
    pub rules: Vec<CoordinationAlertRule>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertCreateResponse {
    pub rule: CoordinationAlertRule,
    /// One-time HMAC secret. Save it — will not be shown again.
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertGetResponse {
    pub rule: CoordinationAlertRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertUpdateResponse {
    pub rule: CoordinationAlertRule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationAlertDeleteResponse {
    pub deleted: bool,
}

// ─── First-touch signal ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ScoutTier {
    #[serde(rename = "S")] S,
    #[serde(rename = "A")] A,
    #[serde(rename = "B")] B,
    #[serde(rename = "C")] C,
}

impl ScoutTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScoutTier::S => "S",
            ScoutTier::A => "A",
            ScoutTier::B => "B",
            ScoutTier::C => "C",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FirstTouchPreset {
    Scout,
    FreshLaunch,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchesParams {
    /// ISO datetime — events strictly newer than this. Polling cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO datetime — events strictly older than this. Pagination cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 1–100. Default: 50 (BASIC capped at 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Single KOL wallet (32–44 base58 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    /// 0–100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_winrate_7d: Option<f64>,
    /// Restrict to scouts of this tier or better. Requires `n_first_touches_30d >= 30`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_scout_tier: Option<ScoutTier>,
    /// Lower the minimum sample size for scout scoring (default 30).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_n_touches: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KolStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_age_max_min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_first_buy_sol: Option<f64>,
    /// Suffix-filter the token mint (e.g. "pump", "bonk").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_suffix: Option<String>,
    /// Shortcut filter sets — `scout` = min_scout_tier=B + min_n_touches=30 + token_age_max_min=60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<FirstTouchPreset>,
    /// Comma-separated includes — currently `followers_4h`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchKol {
    /// Wallet address — only present on Ultra tier.
    #[serde(default)]
    pub wallet: Option<String>,
    pub name: Option<String>,
    pub twitter_url: Option<String>,
    pub winrate_7d: Option<f64>,
    pub strategy: Option<String>,
    pub scout_tier: Option<ScoutTier>,
    /// Same as swarm_3plus_pct on the scout leaderboard.
    pub scout_score: Option<f64>,
    pub n_first_touches_30d: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchEvent {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub token_image_url: Option<String>,
    pub first_buy_at: String,
    pub sol_amount: Option<f64>,
    pub token_amount: Option<f64>,
    pub tx_signature: Option<String>,
    pub token_age_minutes: Option<u32>,
    pub first_kol: FirstTouchKol,
    #[serde(default)]
    pub followers_4h: Option<u32>,
    /// v0.6 (2026-05-06) — market cap (USD) stamped on the exact tx that fired
    /// the first KOL buy, joined via `tx_signature`.
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    /// v0.6 — token price (USD) at the same moment.
    #[serde(default)]
    pub price_usd_at_first_buy: Option<f64>,
    /// v0.6 — current market cap (USD), from `token_prices`.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// v0.6 — current last-trade price (USD).
    #[serde(default)]
    pub last_price_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchesResponse {
    pub events: Vec<FirstTouchEvent>,
    pub count: u32,
    pub next_before: Option<String>,
    pub data_age_seconds: Option<u32>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirstTouchSubscriptionFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_first_buy_sol: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_scout_tier: Option<ScoutTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_n_touches: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscription {
    pub id: String,
    pub name: Option<String>,
    pub filters: FirstTouchSubscriptionFilters,
    pub delivery_mode: CoordinationDeliveryMode,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchSubscriptionCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FirstTouchSubscriptionFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchSubscriptionUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FirstTouchSubscriptionFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<CoordinationDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionListResponse {
    pub subscriptions: Vec<FirstTouchSubscription>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionCreateResponse {
    pub subscription: FirstTouchSubscription,
    /// One-time HMAC secret. Save it — will not be shown again.
    pub webhook_secret: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionGetResponse {
    pub subscription: FirstTouchSubscription,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionUpdateResponse {
    pub subscription: FirstTouchSubscription,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchSubscriptionDeleteResponse {
    pub ok: bool,
}

// ─── Deployer Hunter ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DeployerSortField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAlertsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Cursor — return alerts strictly older than this ISO 8601 timestamp.
    /// Pass `next_before` from previous response. Preferred over `offset` at scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// PRO/ULTRA only. BASIC subscribers receive HTTP 403.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    /// Filter by alert_type (e.g. "new_deploy", "bonded").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<String>,
    /// Filter by alert priority ("high" | "medium" | "low").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Only alerts where at least N KOLs bought the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kol_buys: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAlertStatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<AlertPeriod>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BestTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<BestTokensPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecentBondsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// ISO 8601 datetime — only bonds strictly newer than this timestamp.
    /// Pass `next_since` from the previous response for incremental polling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Filter by deployer reputation tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    /// Only bonds that reached at least this peak market cap (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_mc_min: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTierCounts {
    pub elite: u32,
    pub good: u32,
    pub rising: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStats {
    pub tracked_count: u32,
    pub signals_today: u32,
    pub bonds_detected: u32,
    pub bond_rate: f64,
    pub tiers: DeployerTierCounts,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Shared: DeployerSummary (used by RecentBond, DeployerAlert) ─────────────

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerSummary {
    pub wallet_address: String,
    pub tier: DeployerTier,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub total_bonded: Option<u32>,
    #[serde(default)]
    pub recent_outcomes: Option<String>,
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
    #[serde(default)]
    pub total_tokens_deployed: Option<u32>,
    /// Peak market cap (USD) of this deployer's best token to date. Populated on alert rows.
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerLeaderboardEntry {
    pub id: String,
    pub wallet_address: String,
    pub tier: DeployerTier,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub total_tokens_deployed: u32,
    pub total_bonded: u32,
    #[serde(default)]
    pub last_deploy_at: Option<String>,
    #[serde(default)]
    pub recent_outcomes: Option<String>,
    #[serde(default)]
    pub avg_time_to_bond_minutes: Option<f64>,
    #[serde(default)]
    pub best_token_peak_mc: Option<f64>,
    #[serde(default)]
    pub avg_peak_mc: Option<f64>,
    #[serde(default)]
    pub last_bond_at: Option<String>,
    #[serde(default)]
    pub is_tracked: Option<bool>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerLeaderboardResponse {
    pub deployers: Vec<DeployerLeaderboardEntry>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerToken {
    pub mint: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub bonded: bool,
    pub deployed_at: String,
    pub bonded_at: Option<String>,
    pub peak_market_cap_usd: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerProfile {
    pub wallet: String,
    pub tier: DeployerTier,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub total_deployed: u32,
    pub total_bonded: u32,
    pub last_deploy_at: Option<String>,
    pub first_seen: Option<String>,
    #[serde(default)]
    pub tokens: Option<Vec<DeployerToken>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTokensResponse {
    pub tokens: Vec<DeployerToken>,
    pub count: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KolBuysSummary {
    pub count: u32,
    pub total_sol: f64,
    pub kols: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlert {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    pub alert_type: String,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub created_at: String,
    #[serde(default)]
    pub market_cap_at_alert: Option<f64>,
    pub deployers: DeployerSummary,
    #[serde(default)]
    pub kol_buys: Option<KolBuysSummary>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlertsResponse {
    pub alerts: Vec<DeployerAlert>,
    pub limit: u32,
    pub offset: u32,
    /// Cursor for the next page — pass as `before` to fetch older alerts.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default)]
    pub data_age_seconds: Option<u64>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BondRateStats {
    pub total_deploys: u32,
    pub total_bonded: u32,
    pub rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MultiplierStats {
    pub total_with_mc: u32,
    pub pct_2x: f64,
    pub pct_5x: f64,
    pub pct_10x: f64,
    pub pct_50x: f64,
    pub avg_multiplier: f64,
    pub best_multiplier: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TierStats {
    pub deploys: u32,
    pub bonded: u32,
    pub bond_rate: f64,
    #[serde(default)]
    pub avg_multiplier: Option<f64>,
    pub total_with_mc: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlertStats {
    pub bond_rate: BondRateStats,
    pub multiplier: MultiplierStats,
    pub tiers: HashMap<String, TierStats>,
    pub period: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BestToken {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub bonded_at: String,
    #[serde(default)]
    pub peak_market_cap: Option<f64>,
    #[serde(default)]
    pub mc_at_bond: Option<f64>,
    #[serde(default)]
    pub market_cap_at_alert: Option<f64>,
    #[serde(default)]
    pub mc_multiplier: Option<f64>,
    pub deployer_wallet: String,
    pub deployer_tier: DeployerTier,
    #[serde(default)]
    pub alerted_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BestTokensResponse {
    pub tokens: Vec<BestToken>,
    pub period: String,
    pub limit: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecentBond {
    pub id: String,
    pub token_mint: String,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_image_url: Option<String>,
    pub deployed_at: String,
    pub bonded_at: String,
    #[serde(default)]
    pub time_to_bond_minutes: Option<f64>,
    #[serde(default)]
    pub peak_market_cap: Option<f64>,
    #[serde(default)]
    pub mc_at_bond: Option<f64>,
    pub deployers: DeployerSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecentBondsResponse {
    pub tokens: Vec<RecentBond>,
    pub limit: u32,
    /// Cursor for incremental polling — pass as `since` on the next call to
    /// fetch only newer bonds.
    #[serde(default)]
    pub next_since: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStreak {
    #[serde(rename = "type")]
    pub streak_type: String,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerRollingRate {
    pub window_end: u32,
    pub bond_rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStretch {
    pub start_index: u32,
    pub end_index: u32,
    pub bond_rate: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryData {
    pub current_streak: DeployerStreak,
    pub longest_bond_streak: u32,
    pub longest_fail_streak: u32,
    pub rolling_bond_rates: Vec<DeployerRollingRate>,
    pub trend: String,
    pub avg_days_between_deploys: Option<f64>,
    pub avg_recovery_tokens: Option<f64>,
    pub best_stretch: Option<DeployerStretch>,
    pub worst_stretch: Option<DeployerStretch>,
    pub total_tokens_analyzed: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryDeployer {
    pub wallet_address: String,
    pub total_tokens_deployed: u32,
    pub total_bonded: u32,
    pub bonding_rate: f64,
    pub recent_bond_rate: f64,
    pub tier: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryResponse {
    pub deployer: DeployerTrajectoryDeployer,
    pub trajectory: DeployerTrajectoryData,
}

// ─── Alpha Wallet Intelligence ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlphaSort {
    WinRate,
    Pnl,
    Roi,
}

impl AlphaSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WinRate => "win_rate",
            Self::Pnl => "pnl",
            Self::Roi => "roi",
        }
    }
}

pub type AlphaPeriod = AlertPeriod;

#[derive(Debug, Clone, Default, Serialize)]
pub struct AlphaLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<AlphaPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<AlphaSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bots: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletEntry {
    pub rank: u32,
    pub wallet: String,
    pub tokens_traded: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: Option<f64>,
    pub net_pnl_sol: f64,
    #[serde(default)]
    pub total_sol_bought: Option<f64>,
    #[serde(default)]
    pub total_sol_sold: Option<f64>,
    #[serde(default)]
    pub roi: Option<f64>,
    #[serde(default)]
    pub avg_rank: Option<f64>,
    #[serde(default)]
    pub best_rank: Option<f64>,
    #[serde(default)]
    pub total_buys: Option<u32>,
    #[serde(default)]
    pub total_sells: Option<u32>,
    #[serde(default)]
    pub last_seen: Option<String>,
    #[serde(default)]
    pub bundle_rate: Option<f64>,
    #[serde(default)]
    pub buy_size_stddev: Option<f64>,
    #[serde(default)]
    pub active_hours: Option<f64>,
    #[serde(default)]
    pub bot_confidence: Option<String>,
    /// v0.6 (2026-05-06) — avg market cap (USD) at the moment of each buy in the period.
    /// Null + 0 samples for non-KOL wallets that don't appear in our trade-level dataset.
    #[serde(default)]
    pub avg_entry_mc_usd: Option<f64>,
    #[serde(default)]
    pub entry_mc_samples: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLeaderboardResponse {
    pub leaderboard: Vec<AlphaWalletEntry>,
    pub total: u32,
    pub period: String,
    pub sort: String,
    pub min_tokens: u32,
    pub exclude_bots: bool,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletPosition {
    pub token_mint: String,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub first_buy_at: Option<String>,
    pub last_trade_at: Option<String>,
    pub buy_count: u32,
    pub sell_count: u32,
    pub total_bought_sol: f64,
    pub total_sold_sol: f64,
    pub realized_pnl_sol: f64,
    pub roi_pct: Option<f64>,
    pub result: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletBotSignal {
    pub signal: String,
    pub detail: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletSummary {
    pub tokens_traded: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: Option<f64>,
    pub net_pnl_sol: f64,
    pub total_vol_sol: f64,
    pub roi: Option<f64>,
    pub avg_rank: Option<f64>,
    pub best_rank: Option<f64>,
    pub bundle_rate: f64,
    pub buy_size_stddev: f64,
    pub active_hours: Option<f64>,
    pub bot_confidence: String,
    pub night_only_activity: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletResponse {
    pub wallet: String,
    pub summary: AlphaWalletSummary,
    pub positions: Vec<AlphaWalletPosition>,
    pub bot_signals: Vec<AlphaWalletBotSignal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLinkedWallet {
    pub wallet_address: String,
    pub shared_tokens: u32,
    pub similarity_score: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaLinkedResponse {
    pub wallet: String,
    pub linked_wallets: Vec<AlphaLinkedWallet>,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableBuyer {
    pub rank: u32,
    pub wallet: String,
    pub first_buy_sol: f64,
    pub first_buy_at: Option<String>,
    pub is_bundle: bool,
    pub is_kol: bool,
    pub kol_name: Option<String>,
    pub bot_confidence: Option<String>,
    pub historical_win_rate: Option<f64>,
    pub historical_pnl_sol: Option<f64>,
    pub historical_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableSummary {
    pub known_alpha_wallets: u32,
    pub known_kols: u32,
    pub bundle_buyers: u32,
    pub buyer_quality_score: u32,
    pub confidence: String,
    pub signal: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaCapTableResponse {
    pub mint: String,
    pub buyers: Vec<AlphaCapTableBuyer>,
    pub summary: AlphaCapTableSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityBreakdown {
    pub alpha_wallet_count: u32,
    pub kol_count: u32,
    pub bundle_buyer_count: u32,
    pub avg_historical_win_rate: Option<f64>,
    pub bot_dominated: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityResponse {
    pub mint: String,
    pub score: u32,
    pub confidence: String,
    pub signal: String,
    pub cached_at: String,
    #[serde(default)]
    pub breakdown: Option<AlphaBuyerQualityBreakdown>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlphaBuyerQualityBatchResponse {
    pub tokens: Vec<AlphaBuyerQualityResponse>,
    pub count: u32,
    /// Number of mints served from the shared 5-min LRU cache without a DB query.
    pub cache_hits: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Token intelligence (/token/{mint}) ─────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct TokenKolTopBuyer {
    pub name: String,
    pub sol_amount: f64,
    /// ULTRA only — individual KOL wallet address.
    #[serde(default)]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenKolActivity {
    pub buying_kols: u32,
    pub selling_kols: u32,
    pub net_flow_sol: f64,
    /// "accumulating" | "distributing" | "neutral".
    pub signal: String,
    pub top_buyers: Vec<TokenKolTopBuyer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenDeployerInfo {
    pub wallet: String,
    pub tier: DeployerTier,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    #[serde(default)]
    pub total_deployed: Option<u32>,
    #[serde(default)]
    pub total_bonded: Option<u32>,
    #[serde(default)]
    pub recent_bond_rate: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponseBody {
    pub mint: String,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub price_sol: Option<f64>,
    #[serde(default)]
    pub market_cap: Option<f64>,
    #[serde(default)]
    pub volume_24h_usd: Option<f64>,
    #[serde(default)]
    pub volume_24h_sol: Option<f64>,
    #[serde(default)]
    pub trades_24h: Option<u32>,
    #[serde(default)]
    pub last_trade_at: Option<String>,
    /// When the mint first appeared in our indexer.
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub age_seconds: Option<u64>,
    #[serde(default)]
    pub is_blacklisted: Option<bool>,
    /// "stablecoin" | "wrapped_sol" | "lst" | "rug" | custom category when blacklisted.
    #[serde(default)]
    pub blacklist_category: Option<String>,
    #[serde(default)]
    pub deployer: Option<TokenDeployerInfo>,
    pub kol_activity: TokenKolActivity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub token: TokenResponseBody,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenBatchResponse {
    pub tokens: Vec<TokenResponseBody>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MintBatchRequest {
    /// 1–50 base58 Solana token mint addresses.
    pub mints: Vec<String>,
}

// ─── Wallet Tracker ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WatchlistAddParams {
    pub wallet_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WatchlistUpdateParams {
    /// New label for the wallet, or `None` to clear it.
    pub label: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletEntry {
    pub wallet_address: String,
    pub label: Option<String>,
    pub added_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistAddResponse {
    pub wallet_address: String,
    pub label: Option<String>,
    pub added_at: String,
    pub remaining: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchlistResponse {
    pub wallets: Vec<WalletEntry>,
    pub count: u32,
    pub limit: u32,
    pub remaining: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerEvent {
    pub id: String,
    pub wallet_address: String,
    pub label: Option<String>,
    pub event_type: WalletTrackerEventType,
    pub action: WalletTrackerAction,
    pub block_time: i64,
    pub block_time_iso: String,
    pub token_mint: Option<String>,
    pub token_symbol: Option<String>,
    pub token_name: Option<String>,
    pub sol_amount: f64,
    pub token_amount: Option<f64>,
    pub price_per_token_sol: Option<f64>,
    pub counterparty: Option<String>,
    pub tx_signature: Option<String>,
    pub program: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletTrackerTradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WalletTrackerAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<WalletTrackerEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerTradesResponse {
    pub events: Vec<WalletTrackerEvent>,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerWalletStats {
    pub wallet_address: String,
    pub label: Option<String>,
    pub swap_count: u32,
    pub buys: u32,
    pub sells: u32,
    pub sol_bought: f64,
    pub sol_sold: f64,
    pub last_event_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WalletTrackerSummaryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<WalletTrackerSummaryPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerSummaryResponse {
    pub wallets: Vec<WalletTrackerWalletStats>,
    pub period: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletTrackerDeleteResponse {
    pub success: bool,
}

// ─── Tools ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize)]
pub struct ToolsSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    pub name: String,
    pub slug: String,
    pub tagline: String,
    pub website_url: String,
    #[serde(default)]
    pub logo_url: Option<String>,
    pub categories: Vec<String>,
    #[serde(default)]
    pub pricing_model: Option<String>,
    #[serde(default)]
    pub average_rating: Option<f64>,
    #[serde(default)]
    pub review_count: Option<u32>,
    #[serde(default)]
    pub health_score: Option<f64>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolsSearchResponse {
    pub tools: Vec<Tool>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Streaming ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct StreamToken {
    pub token: String,
    pub expires_at: String,
    #[serde(default)]
    pub next_refresh_at: Option<String>,
    pub ws_url: String,
    /// Only present for ULTRA-tier subscribers.
    #[serde(default)]
    pub dex_ws_url: Option<String>,
    pub usage: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

// ─── Webhooks ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WebhookCreateParams {
    pub url: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WebhookUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    pub id: i64,
    pub url: String,
    pub events: Vec<String>,
    pub filters: Option<HashMap<String, serde_json::Value>>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookListResponse {
    pub webhooks: Vec<Webhook>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebhookDeleteResponse {
    pub success: bool,
}
