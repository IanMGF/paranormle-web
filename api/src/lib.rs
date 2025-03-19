pub mod episode_history;

#[cfg(debug_assertions)]
pub const ENVIRONMENT: &str = "debug";
#[cfg(not(debug_assertions))]
pub const ENVIRONMENT: &str = "release";
