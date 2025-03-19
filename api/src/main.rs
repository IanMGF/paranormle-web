use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};

use common::{episode::Episode, EPISODES_JSON};
use rand::Rng;
use tower_http::cors::{Any, CorsLayer};

use std::collections::HashSet;

pub async fn get_day_episode(episodes: &[Episode]) -> usize {
    let used_eps = api::episode_history::get_episode_idx_history().await.unwrap();

    let opt_registered_ep = used_eps.iter().find(|entry| entry.day == common::get_day_offset());
    if let Some(registered_ep) = opt_registered_ep {
        return registered_ep.episode_idx;
    }

    let unused_eps: Vec<usize> = {
        let used_idxs = used_eps.iter().map(|entry| entry.episode_idx).collect::<HashSet<usize>>();
        let set = (0..episodes.len()).collect::<HashSet<usize>>();
        set.difference(&used_idxs).cloned().collect::<Vec<usize>>()
    };
    
    let mapped_idx = rand::rng().random_range(0..unused_eps.len());
    let episode_idx = unused_eps[mapped_idx];

    api::episode_history::register_day_episode(episode_idx).await.unwrap();

    episode_idx
}

async fn day_episode() -> String {
    let episodes: Vec<Episode> = serde_json::from_str(EPISODES_JSON).unwrap();
    let episode_idx: usize = get_day_episode(&episodes).await;

    episode_idx.to_string()
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var("API_URL", secrets.get("API_URL").unwrap());
    std::env::set_var("DB_URL", secrets.get("DB_URL").unwrap());

    let cors_origin = if cfg!(debug_assertions) {
        "http://127.0.0.1:8080"
    } else {
        "https://ianmgf.github.io/paranormle-web/"
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .expose_headers(Any)
        .allow_origin(cors_origin.parse::<HeaderValue>().unwrap());

    let router = Router::new()
        .route("/episode", get(day_episode))
        .layer(cors);

    Ok(router.into())
}
