use axum::{http::{HeaderValue, Method}, routing::get, Router};
use chrono::NaiveDate;
use common::{episode::Episode, EPISODES_JSON};
use mysql::{prelude::Queryable, Pool, PooledConn};
use tower_http::cors::{Any, CorsLayer};
use rand::Rng;

use std::collections::HashSet;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
struct EpisodeHistoryEntry {
    day: usize,
    episode_idx: usize,
}

#[cfg(debug_assertions)]
const ENVIRONMENT : &str = "debug";
#[cfg(not(debug_assertions))]
const ENVIRONMENT : &str = "release";

async fn get_episode_idx_history() -> Result<HashSet<EpisodeHistoryEntry>, Box<dyn std::error::Error>> {
    let url = env!("DB_URL");
    let pool = Pool::new(url)?;

    let mut conn: PooledConn = pool.get_conn()?;
    let registered_days = conn.query_map(
        format!("SELECT day, episode_idx FROM episode_history WHERE env='{}' ORDER BY day DESC LIMIT 30", self::ENVIRONMENT),
        |(day, episode_idx)| {
            EpisodeHistoryEntry { day, episode_idx }
        },
    )?;
    
    Ok(registered_days.into_iter().collect())
}

fn get_day_offset() -> usize {
    let day_zero: NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
    let curr_date = chrono::Local::now().date_naive();

    curr_date.signed_duration_since(day_zero).num_days() as usize
}

async fn register_day_episode(episode_idx: usize) -> Result<(), Box<dyn std::error::Error>> {
    let url = env!("DB_URL");
    let pool = Pool::new(url)?;
    
    let mut conn: PooledConn = pool.get_conn()?;
    
    conn.exec_drop(
        "INSERT INTO episode_history (episode_idx, day, env) VALUES (?, ?, ?)",
        (episode_idx, get_day_offset(), ENVIRONMENT),
    )?;
    
    Ok(())
}

pub async fn get_day_episode(episodes: &[Episode]) -> usize {
    let used_eps = get_episode_idx_history().await.unwrap();

    if let Some(registered_ep) = used_eps.iter().find(|entry| entry.day == get_day_offset()) {
        return registered_ep.episode_idx;
    }
    
    let unused_eps: Vec<usize> = {
        (0..episodes.len())
            .filter(
                |idx| {
                    !used_eps
                        .iter()
                        .map(|ep| ep.episode_idx)
                        .any(|used_idx| used_idx == *idx)
                }
            )
            .collect()
    };
    
    let mapped_idx = rand::rng().random_range(0..unused_eps.len());
    let episode_idx = unused_eps[mapped_idx];
    
    register_day_episode(episode_idx).await.unwrap();
    
    episode_idx
}

async fn day_episode() -> String {
    let episodes: Vec<Episode> = serde_json::from_str(EPISODES_JSON).unwrap();
    let episode_idx: usize = get_day_episode(&episodes).await;
    
    episode_idx.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cors_origin = if cfg!(debug_assertions) {
        "http://127.0.0.1:8080"
    } else {
        "https://ianmgf.github.io/paranormle-web/"
    };
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .expose_headers(Any)
        .allow_origin(cors_origin.parse::<HeaderValue>().unwrap());
    
    let router = Router::new().route("/episode", get(day_episode)).layer(cors);

    Ok(router.into())
}
