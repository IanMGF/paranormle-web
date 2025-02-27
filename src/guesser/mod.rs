mod guess;
mod info_piece;
mod input;

use crate::episode::Episode;
use guess::Guess;
use input::guess_callback;
use std::hash::{DefaultHasher, Hash, Hasher};
use yew::prelude::*;

#[function_component(Header)]
fn titles() -> Html {
    html! {
        <div class={ "titles-wrapper" }>
            <div class={ "titles" }>
                <div class={ "neutral" }>{ "Título" }</div>
                <div class={ "neutral" }>{ "Nro do Episódio" }</div>
                <div class={ "neutral" }>{ "Campanha" }</div>
                <div class={ "neutral" }>{ "Duração" }</div>
                <div class={ "neutral" }>{ "Ano" }</div>
                <div class={ "neutral" }>{ "Jogadores" }</div>
                <div class={ "neutral" }>{ "Tem Cinematic" }</div>
            </div>
        </div>
    }
}

#[function_component(Guesser)]
pub fn guesser() -> Html {
    const EPISODES: &str = include_str!("../../res/data/episodes.json");

    let episodes: Vec<Episode> = serde_json::from_str(EPISODES).unwrap();

    let guesses: UseStateHandle<Vec<Episode>> = use_state(Vec::new);
    let has_guessed: UseStateHandle<bool> = use_state(|| false);

    let today_ep = {
        let mut hasher = DefaultHasher::new();
        let date = chrono::Local::now().date_naive();
        date.hash(&mut hasher);

        let today_idx = hasher.finish() as usize % episodes.len();
        &episodes[today_idx]
    };

    let input_callback = guess_callback(
        &episodes,
        today_ep.clone(),
        guesses.clone(),
        has_guessed.clone(),
    );
    let event_callback = guess_callback(
        &episodes,
        today_ep.clone(),
        guesses.clone(),
        has_guessed.clone(),
    );

    let options = episodes
        .iter()
        .map(|ep| html! { <option value={ ep.title.clone() } /> })
        .collect::<Html>();

    let guesses_list = html! {
        guesses
            .iter()
            .map(|ep| html! {
                <li class="guess-wrapper">
                <Guess episode={ ep.clone() } correct={ today_ep.clone() } />
                </li>
            })
            .rev()
            .collect::<Html>()
    };

    html! {
        <>
            < datalist id="episodes" > { options } </datalist>
            <input
                onkeyup={input_callback.clone()}
                oninput={event_callback.clone()}
                list={ "episodes" }
                type={ "text" }
                placeholder={ "Adivinhe o episódio..." }
                id="episode-guess"
                style="width:895px;"
                disabled={ *has_guessed }
            />

            < Header/ >
            <ul id="guesses">
                { guesses_list }
            </ul>
        </>
    }
}
