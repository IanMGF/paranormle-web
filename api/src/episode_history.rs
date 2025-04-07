use std::collections::HashSet;

use mysql::{prelude::Queryable, Pool, PooledConn};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    pub day: usize,
    pub episode_idx: usize,
}

pub struct EpisodeHistory(PooledConn);

impl EpisodeHistory {
    pub fn new() -> Result<EpisodeHistory, mysql::Error> {
        const URL: &str = env!("DB_URL");
        let pool = Pool::new(URL)?;

        let conn: PooledConn = pool.get_conn()?;

        Ok(EpisodeHistory(conn))
    }

    pub async fn get_episode_idx_history(&mut self) -> Result<HashSet<HistoryEntry>, mysql::Error> {
        let conn = &mut self.0;

        let registered_days = conn.exec_map(
            "
                SELECT day, episode_idx 
                FROM episode_history WHERE env=? 
                ORDER BY day DESC LIMIT 30
            ", (crate::ENVIRONMENT,),
            |(day, episode_idx)| HistoryEntry { day, episode_idx },
        )?;

        Ok(registered_days.into_iter().collect())
    }

    pub async fn register_day_episode(&mut self, episode_idx: usize) -> Result<(), mysql::Error> {
        let conn = &mut self.0;

        conn.exec_drop(
            "
                INSERT INTO episode_history
                (episode_idx, day, env) VALUES (?, ?, ?)
            ", (episode_idx, common::get_day_offset(), crate::ENVIRONMENT),
        )?;

        Ok(())
    }
}
