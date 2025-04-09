use std::collections::HashSet;

use mysql::{prelude::Queryable, Pool, PooledConn};

use super::{EpisodeHistory, HistoryEntry};

pub struct MySQLDB(PooledConn);

impl MySQLDB {
    pub fn new() -> Result<Self, mysql::Error> {
        const URL: &str = env!("DB_URL");
        let pool = Pool::new(URL)?;

        let conn: PooledConn = pool.get_conn()?;

        Ok(Self(conn))
    }
}

impl EpisodeHistory for MySQLDB {
    type Error = mysql::Error;

    async fn get_episode_idx_history(&mut self) -> Result<HashSet<HistoryEntry>, mysql::Error> {
        let conn = &mut self.0;

        let registered_days = conn.exec_map(
            "
                SELECT day, episode_idx 
                FROM episode_history WHERE env=? 
                ORDER BY day DESC LIMIT 30
            ",
            (Into::<&str>::into(crate::ENVIRONMENT),),
            |(day, episode_idx)| HistoryEntry { day, episode_idx },
        )?;

        Ok(registered_days.into_iter().collect())
    }

    async fn register_day_episode(&mut self, episode_idx: usize) -> Result<(), mysql::Error> {
        let conn = &mut self.0;

        conn.exec_drop(
            "
                INSERT INTO episode_history
                (episode_idx, day, env) VALUES (?, ?, ?)
            ",
            (
                episode_idx,
                common::get_day_offset(),
                Into::<&str>::into(crate::ENVIRONMENT),
            ),
        )?;

        Ok(())
    }
}
