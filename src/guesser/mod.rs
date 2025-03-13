mod guess;
mod info_piece;
mod input;

use crate::episode::Episode;
use guess::Guess;
use input::Input;
use stylist::style;
use std::{hash::{DefaultHasher, Hash, Hasher}, rc::Rc};
use yew::prelude::*;

#[function_component(Header)]
fn header() -> Html {
    let header_style = style!("
        width: max-content;
        position: relative;
        translate: -50%;
        left: 50vw;
        
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: row;
        border-radius: 10px;
    ").expect("Failed to create Guesser Header style");
    
    let header_unit_style = style!("
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
    ").expect("Failed to create Guesser Header Unit style");
    
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

const EPISODES: &str = include_str!("../../res/data/episodes.json");

#[function_component(Guesser)]
pub fn guesser() -> Html {
    let episodes: Rc<Vec<Episode>> = use_memo((), |_| serde_json::from_str(EPISODES).unwrap());
    let today_ep: Rc<Episode> = use_memo((), |_| {
        let mut hasher = DefaultHasher::new();
        let date = chrono::Local::now().date_naive();
        date.hash(&mut hasher);

        #[cfg(debug_assertions)]
        "dbg_ver".hash(&mut hasher);

        let today_idx = hasher.finish() as usize % (*episodes).len();
        (*episodes)[today_idx].clone()
    });

    let guesses: UseStateHandle<Vec<Episode>> = use_state(Vec::new);
    let has_guessed: UseStateHandle<bool> = use_state(|| false);

    let guesses_list = html! {
        guesses
            .iter()
            .map(|ep| html! {
                <li>
                <Guess episode={ ep.clone() } correct={ today_ep.clone() } />
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
