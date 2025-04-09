use std::sync::Arc;

use chrono_tz::America::Sao_Paulo;
use episode::Episode;
use lazy_static::lazy_static;
use chrono::TimeZone;

pub mod episode;

pub const NON_REPEATING_PERIOD: u64 = 30;
pub const EPISODES_JSON: &str = include_str!("../data/episodes.json");

pub fn get_day_offset() -> usize {
    let day_zero = Sao_Paulo
        .with_ymd_and_hms(2025, 2, 20, 0, 0, 0)
        .unwrap();
    let curr_date = chrono::Local::now();

    curr_date.signed_duration_since(day_zero).num_days() as usize
}

lazy_static! {
    pub static ref EPISODES_LIST: Arc<Vec<Episode>> =
        Arc::new(serde_json::from_str(EPISODES_JSON).unwrap());
}
