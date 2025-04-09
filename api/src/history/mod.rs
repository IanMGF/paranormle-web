pub mod firebase;
pub mod mysql;

use std::collections::HashSet;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct HistoryEntry {
    pub day: usize,
    pub episode_idx: usize,
}

pub trait EpisodeHistory {
    type Error;

    fn get_episode_idx_history(
        &mut self,
    ) -> impl std::future::Future<Output = Result<HashSet<HistoryEntry>, Self::Error>>;
    fn register_day_episode(
        &mut self,
        episode_idx: usize,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>>;
}
