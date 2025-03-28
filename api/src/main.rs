use api::episode_history;
use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};

use common::episode::Episode;
use rand::Rng;
use tower_http::cors::{Any, CorsLayer};

use std::collections::HashSet;

pub async fn get_day_episode(episodes: &[Episode]) -> usize {
    let invalid_eps = episode_history::get_episode_idx_history().await.unwrap();

    let opt_registered_ep = invalid_eps.iter().find(|entry| entry.day == common::get_day_offset());
    if let Some(registered_ep) = opt_registered_ep {
        return registered_ep.episode_idx;
    }

    let invalid_eps: Vec<usize> = {
        let invalid_idxs = invalid_eps.iter().map(|entry| entry.episode_idx).collect::<HashSet<usize>>();
        let set = (0..episodes.len()).collect::<HashSet<usize>>();
        set.difference(&invalid_idxs).cloned().collect::<Vec<usize>>()
    };
    
    let mapped_idx = rand::rng().random_range(0..invalid_eps.len());
    let episode_idx = invalid_eps[mapped_idx];

    episode_history::register_day_episode(episode_idx).await.unwrap();

    episode_idx
}

async fn day_episode() -> String {
    let episode_idx: usize = get_day_episode(&common::EPISODES_LIST).await;

    episode_idx.to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cors_origin = env!("CORS_ORIGIN");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .expose_headers(Any)
        .allow_origin(cors_origin.parse::<HeaderValue>().unwrap());

    let router = Router::new()
        .route("/episode", get(day_episode))
        .layer(cors);

    Ok(router.into())
}
