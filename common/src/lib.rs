use chrono::NaiveDate;

pub mod episode;

pub const NON_REPEATING_PERIOD: u64 = 64;
pub const EPISODES_JSON: &str = include_str!("../data/episodes.json");

pub fn get_day_offset() -> usize {
    let day_zero: NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
    let curr_date = chrono::Local::now().date_naive();

    curr_date.signed_duration_since(day_zero).num_days() as usize
}