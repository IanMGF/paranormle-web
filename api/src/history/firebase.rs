// POST
// https://paranormle-db-default-rtdb.firebaseio.com/episodes/
// 75

use std::collections::HashMap;

use firebase_rs::{Firebase, RequestError, UrlParseError};

use crate::ENVIRONMENT;

use super::{EpisodeHistory, HistoryEntry};

pub struct FirebaseDB(Firebase);

const FIREBASE_URI: &str = match ENVIRONMENT {
    crate::Environment::DEBUG => "episodes_dev",
    crate::Environment::RELEASE => "episodes",
};
impl FirebaseDB {
    pub fn new() -> Result<FirebaseDB, UrlParseError> {
        const FIREBASE_URL: &str = env!("FIREBASE_URL");
        const FIREBASE_AUTH_KEY: &str = ""; // env!("FIREBASE_AUTH_KEY");
        Firebase::auth(FIREBASE_URL, FIREBASE_AUTH_KEY).map(Self)
    }
}

impl EpisodeHistory for FirebaseDB {
    type Error = RequestError;

    async fn get_episode_idx_history(
        &mut self,
    ) -> Result<std::collections::HashSet<super::HistoryEntry>, Self::Error> {
        let episodes_firebase = self.0.at(FIREBASE_URI);
        let episodes = episodes_firebase.get::<Vec<HistoryEntry>>().await?;

        Ok(episodes.into_iter().collect())
    }

    async fn register_day_episode(&mut self, episode_idx: usize) -> Result<(), Self::Error> {
        let episodes_firebase = self.0.at(FIREBASE_URI);

        let mut episodes: Vec<HistoryEntry> = episodes_firebase.get::<Vec<HistoryEntry>>().await?;

        let day = common::get_day_offset();

        episodes.push(HistoryEntry { day, episode_idx });
        let episodes_obj: HashMap<usize, HistoryEntry> =
            { episodes.into_iter().enumerate().collect() };

        episodes_firebase.update(&episodes_obj).await?;

        Ok(())
    }
}
