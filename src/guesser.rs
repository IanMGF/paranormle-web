use crate::{episode::Episode, theme::Theme};
use chrono::Datelike;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct GuessProp {
    episode: Episode,
    correct: Episode,
}

fn ord_to_css_class(ord: std::cmp::Ordering) -> &'static str {
    match ord {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    }
}

fn eq_to_css_class(eq: bool) -> &'static str {
    match eq {
        true => "correct",
        false => "incorrect",
    }
}

#[function_component(Guess)]
pub fn guess(guess: &GuessProp) -> Html {
    let theme = Theme::gen_day_element();
    let correct_guess_css = format!(
        "
        background-color: {0}ee;
        -webkit-box-shadow:0px 0px 5px 3px {0}cc;
        -moz-box-shadow: 0px 0px 5px 3px {0}cc;
        box-shadow: 0px 0px 5px 3px {0}cc;
    ",
        theme.get_guess_color()
    );

    let guess_css = match guess.episode == guess.correct {
        true => correct_guess_css,
        false => String::from("background-color: #17171777;"),
    };

    let ep_number_cls = ord_to_css_class(Ord::cmp(&guess.episode.number, &guess.correct.number));
    let campaign_cls = eq_to_css_class(guess.episode.campaign == guess.correct.campaign);
    let date_cls = ord_to_css_class(Ord::cmp(&guess.episode.date.year(), &guess.correct.date.year()));
    let dur_cls = ord_to_css_class(Ord::cmp(&guess.episode.duration, &guess.correct.duration));
    let player_count_cls =
        ord_to_css_class(Ord::cmp(&guess.episode.players, &guess.correct.players));
    let cinematic_cls = eq_to_css_class(guess.episode.has_cinematic == guess.correct.has_cinematic);

    let thumb_url = format!("res/{}", guess.episode.cover_path);

    let title = guess.episode.title.as_str();
    let ep_number = guess.episode.number.to_string();
    let campaign = guess.episode.campaign.as_str();
    let dur = guess.episode.dur_fmt();
    let date = guess.episode.date.year().to_string();
    let player_count = guess.episode.players - 1;
    let has_cinematic = match guess.episode.has_cinematic {
        true => "Sim",
        false => "Não",
    };

    let fade_in = |dur: f64| format!("animation-delay: {}s;", dur);
    let animation_delay = 3.13 / 6.0;

    html! {
        <div class={ "guess" } style={ guess_css }>
            <div class={ "neutral thumbnail" } style={ fade_in(0.0) }>
                <img class={"thumb-img"} src={ thumb_url } />
                <span class={"title"}>{ title }</span>
            </div>
            <div class={ ep_number_cls } style={ fade_in(1.0 * animation_delay) }>{ ep_number }</div>
            <div class={ campaign_cls } style={ fade_in(2.0 * animation_delay) }>{ campaign }</div>
            <div class={ dur_cls } style={ fade_in(3.0 * animation_delay) }>{ dur }</div>
            <div class={ date_cls } style={ fade_in(4.0 * animation_delay) }>{ date }</div>
            <div class={ player_count_cls } style={ fade_in(5.0 * animation_delay) }>{ player_count } { " jogadores" }</div>
            <div class={ cinematic_cls } style={ fade_in(6.0 * animation_delay) }>{ has_cinematic }</div>
        </div>
    }
}

#[function_component(Titles)]
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
    const EPISODES: &str = include_str!("../res/data/episodes.json");

    let guesses: UseStateHandle<Vec<Episode>> = use_state(Vec::new);
    let episodes: Vec<Episode> = serde_json::from_str(EPISODES).unwrap();

    // let today_idx = {
    //     let crypto = web_sys::window().unwrap().crypto().unwrap();
    //     let mut buf = [0u8; 4];
    //     crypto.get_random_values_with_u8_array(&mut buf).unwrap();
    //     let seed = buf[0] as usize;
    //     seed % episodes.len()
    // };
    
    let today_idx = (1234u64).wrapping_pow(413u32) as usize % episodes.len();

    let today_ep = episodes[today_idx].clone();

    let on_key_up = Callback::from({
        let guesses = guesses.clone();
        let correct = today_ep.clone();
        let episodes = episodes.clone();

        move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let guess = input.value();

            if guesses.iter().any(|ep| ep == &correct) {
                return;
            }
            if guesses.iter().any(|ep| ep.title == guess) {
                return;
            }

            let Some(ep) = episodes.iter().find(|ep| ep.title == guess) else {
                return;
            };

            if e.key() == "Enter" {
                let mut g = guesses.to_vec();
                g.push(ep.clone());
                guesses.set(g);
            }
        }
    });

    let on_input = Callback::from({
        let episodes = episodes.clone();
        let guesses = guesses.clone();
        let correct = today_ep.clone();

        move |e: InputEvent| {
            let Some(guess) = e.data() else {
                return;
            };
            if guesses.iter().any(|ep| ep == &correct) {
                return;
            }
            if guesses.iter().any(|ep| ep.title == guess) {
                return;
            }

            let Some(ep) = episodes.iter().find(|ep| ep.title == guess) else {
                return;
            };

            let mut g = guesses.to_vec();
            g.push(ep.clone());
            guesses.set(g);
        }
    });

    let options = html! { episodes.iter().map(|ep| html! { <option value={ ep.title.clone() } /> }).collect::<Html>() };
    let list = html! {
        guesses.iter().map(|ep| html! {
            <li class="guess-wrapper"> <Guess episode={ ep.clone() } correct={ today_ep.clone() }></Guess> </li>
        }).rev().collect::<Html>()
    };

    html! {
        <>
            <input onkeyup={on_key_up.clone()} oninput={on_input.clone()} list={ "episodes" } type={ "text" } placeholder={ "Adivinhe o episódio..." } id="episode-guess" style="width:895px;" />
            <datalist id="episodes"> { options } </datalist>
            < Titles/ >
            <ul id="guesses"> {
                list
            } </ul>
        </>
    }
}
