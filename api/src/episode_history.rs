use std::collections::HashSet;

use mysql::{prelude::Queryable, Pool, PooledConn};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    pub day: usize,
    pub episode_idx: usize,
}

pub async fn get_episode_idx_history(
) -> Result<HashSet<HistoryEntry>, Box<dyn std::error::Error>> {
    let url = env!("DB_URL");
    let pool = Pool::new(url)?;

    let mut conn: PooledConn = pool.get_conn()?;
    let registered_days = conn.query_map(
        format!("SELECT day, episode_idx FROM episode_history WHERE env='{}' ORDER BY day DESC LIMIT 30", crate::ENVIRONMENT),
        |(day, episode_idx)| { HistoryEntry { day, episode_idx } },
    )?;

    Ok(registered_days.into_iter().collect())
}

pub async fn register_day_episode(episode_idx: usize) -> Result<(), Box<dyn std::error::Error>> {
    let url = env!("DB_URL");
    let pool = Pool::new(url)?;

    let mut conn: PooledConn = pool.get_conn()?;

    conn.exec_drop(
        "INSERT INTO episode_history (episode_idx, day, env) VALUES (?, ?, ?)",
        (episode_idx, common::get_day_offset(), crate::ENVIRONMENT),
    )?;

    Ok(())
}