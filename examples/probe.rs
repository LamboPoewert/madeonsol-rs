use madeonsol::MadeOnSol;
use madeonsol::types::*;

#[tokio::main]
async fn main() {
    let key = std::env::var("MADEONSOL_API_KEY").expect("MADEONSOL_API_KEY required");
    let c = MadeOnSol::new(key).unwrap();

    macro_rules! probe {
        ($label:expr, $call:expr) => {
            match $call.await {
                Ok(_) => println!("  OK   {}", $label),
                Err(e) => println!("  FAIL {}  -- {}", $label, e),
            }
        };
    }

    println!("--- KOL ---");
    probe!("kol.feed", c.kol.feed(&KolFeedParams { limit: Some(2), ..Default::default() }));
    probe!("kol.leaderboard", c.kol.leaderboard(&KolLeaderboardParams { limit: Some(2), ..Default::default() }));
    probe!("kol.trending_tokens", c.kol.trending_tokens(&KolTrendingParams { limit: Some(2), ..Default::default() }));
    probe!("kol.hot_tokens", c.kol.hot_tokens(&KolHotTokensParams { limit: Some(2), ..Default::default() }));
    probe!("kol.alerts", c.kol.alerts(&KolAlertsParams { limit: Some(2), ..Default::default() }));
    probe!("kol.coordination", c.kol.coordination(&KolCoordinationParams { limit: Some(2), ..Default::default() }));
    probe!("kol.pairs", c.kol.pairs(&KolPairsParams { limit: Some(2), ..Default::default() }));

    println!("--- DEPLOYER ---");
    probe!("deployer.stats", c.deployer.stats());
    probe!("deployer.leaderboard", c.deployer.leaderboard(&DeployerLeaderboardParams { limit: Some(2), ..Default::default() }));
    probe!("deployer.recent_bonds", c.deployer.recent_bonds(&RecentBondsParams { limit: Some(2), ..Default::default() }));
    probe!("deployer.alerts", c.deployer.alerts(&DeployerAlertsParams { limit: Some(2), ..Default::default() }));
    probe!("deployer.alert_stats", c.deployer.alert_stats(&DeployerAlertStatsParams::default()));
    probe!("deployer.best_tokens", c.deployer.best_tokens(&BestTokensParams { limit: Some(2), ..Default::default() }));

    println!("--- ALPHA ---");
    probe!("alpha.leaderboard", c.alpha.leaderboard(&AlphaLeaderboardParams::default()));

    println!("--- TOOLS ---");
    probe!("tools.search", c.tools.search(&ToolsSearchParams { q: Some("raydium".into()), limit: Some(2), ..Default::default() }));

    println!("--- WALLET TRACKER ---");
    probe!("wallet_tracker.watchlist", c.wallet_tracker.watchlist());

    println!("--- WEBHOOKS ---");
    probe!("webhooks.list", c.webhooks.list());

    println!("--- COORDINATION ALERTS ---");
    probe!("coordination_alerts.list", c.coordination_alerts.list());

    println!("--- STREAM ---");
    probe!("stream.get_token", c.stream.get_token());
}
