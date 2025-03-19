mod guess;
mod info_piece;
mod input;

use common::{episode::Episode, EPISODES_JSON};
use guess::Guess;
use input::Input;
use std::rc::Rc;
use stylist::style;
use yew::prelude::*;

use crate::daily_episode;

#[function_component(Header)]
fn header() -> Html {
    let header_style = style!(
        "
        width: max-content;
        position: relative;
        translate: -50%;
        left: 50vw;
        
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: row;
        border-radius: 10px;
    "
    )
    .expect("Failed to create Guesser Header style");

    let header_unit_style = style!(
        "
        font-family: AdobeHebrew;
        font-weight: bolder;
        font-size: 18px;
        width: 175px;
        height: 75px;
        margin: 5px 5px 0px 5px;  
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        flex-direction: column;
        color: white;
        background-size: contain;
        border-radius: 5px;
        background-color: #000000cc;
    "
    )
    .expect("Failed to create Guesser Header Unit style");

    let header_unit_class = header_unit_style.get_class_name().to_owned();

    html! {
        <div class={ header_style.get_class_name().to_owned() }>
            <div class={ &header_unit_class }>{ "Título" }</div>
            <div class={ &header_unit_class }>{ "Nro do Episódio" }</div>
            <div class={ &header_unit_class }>{ "Campanha" }</div>
            <div class={ &header_unit_class }>{ "Duração" }</div>
            <div class={ &header_unit_class }>{ "Ano" }</div>
            <div class={ &header_unit_class }>{ "Jogadores" }</div>
            <div class={ &header_unit_class }>{ "Tem Cinematic" }</div>
        </div>
    }
}

#[function_component(Guesser)]
pub fn guesser() -> Html {
    let episodes: Rc<Vec<Episode>> = use_memo((), |_| serde_json::from_str(EPISODES_JSON).unwrap());
    let episode: UseStateHandle<Option<Rc<Episode>>> = use_state(|| None);

    let guesses: UseStateHandle<Vec<Rc<Episode>>> = use_state(Vec::new);
    let has_guessed: UseStateHandle<bool> = use_state(|| false);

    use_effect_with((), {
        let episodes = episodes.clone();
        let episode = episode.clone();

        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let episode_idx = daily_episode::get_day_episode().await;
                episode.set(episode_idx.ok().map(|idx| Rc::new(episodes[idx].clone())));
            });
        }
    });

    let Some(today_ep): Option<Rc<Episode>> = (*episode).clone() else {
        return html! {
            <span class="centered" style="color: white; font-size: 28px; font-family: AdobeHebrew; font-weight: bold;">
                { "Recebendo os Sinais..." }
            </span>
        };
    };

    let guesses_list = html! {
        guesses
            .iter()
            .map(|ep| html! {
                <li style="list-style-type: none;">
                <Guess episode={ ep } correct={ &today_ep } />
                </li>
            })
            .rev()
            .collect::<Html>()
    };

    html! {
        <>
            <Input
                episode_list={ episodes.clone() }
                episode_of_the_day={ today_ep.clone() }
                guesses={ guesses.clone() }
                has_guessed={ has_guessed.clone() }
            />

            {
                if guesses.is_empty() { html! { } }
                else { html! { < Header/ > } }
            }
            <ul id="guesses" class="centered">
                { guesses_list }
            </ul>
        </>
    }
}
