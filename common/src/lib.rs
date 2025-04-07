use std::sync::Arc;

use episode::Episode;
use lazy_static::lazy_static;
use chrono::NaiveDate;

pub mod episode;

pub const NON_REPEATING_PERIOD: u64 = 30;
pub const EPISODES_JSON: &str = include_str!("../data/episodes.json");

pub fn get_day_offset() -> usize {
    let day_zero: NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
    let curr_date = chrono::Local::now().date_naive();

    curr_date.signed_duration_since(day_zero).num_days() as usize
}

lazy_static! {
    pub static ref EPISODES_LIST: Arc<Vec<Episode>> =
        Arc::new(serde_json::from_str(EPISODES_JSON).unwrap());
}
