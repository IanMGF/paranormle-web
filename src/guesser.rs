use crate::{episode::Episode, theme};
use chrono::Datelike;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct GuessProp {
    episode: Episode,
    correct: Episode,
}

#[function_component(Guess)]
pub fn guess(guess: &GuessProp) -> Html {
    let theme = theme::get_day_guess_color();

    let guess_css = match guess.episode == guess.correct {
        true => format!(
            "
            background-color: {0}ee;
            -webkit-box-shadow:0px 0px 5px 3px {0}cc;
            -moz-box-shadow: 0px 0px 5px 3px {0}cc;
            box-shadow: 0px 0px 5px 3px {0}cc;
        ",
            theme
        ),
        false => String::from("background-color: #17171777;"),
    };

    let campaign_cls = match guess.episode.campaign == guess.correct.campaign {
        true => "correct",
        false => "incorrect",
    };

    let date_cls = match Ord::cmp(&guess.episode.date.year(), &guess.correct.date.year()) {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    };

    let dur_cls = match Ord::cmp(&guess.episode.duration, &guess.correct.duration) {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    };

    let player_count_cls = match Ord::cmp(&guess.episode.players, &guess.correct.players) {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    };

    let cinematic_cls = match guess.episode.has_cinematic == guess.correct.has_cinematic {
        true => "correct",
        false => "incorrect",
    };

    let thumb_url = format!("res/{}", guess.episode.cover_path);
    let title = guess.episode.title.as_str();
    let campaign = guess.episode.campaign.as_str();
    let dur = guess.episode.dur_fmt();
    let date = guess.episode.date.year().to_string();
    let player_count = guess.episode.players - 1;
    let has_cinematic = match guess.episode.has_cinematic {
        true => "Tem Cinematic(s)",
        false => "Não tem Cinematic(s)",
    };

    let fade_in = |dur: f64| format!("animation-delay: {}s;", dur);

    let animation_delay = 3.13 / 5.0;

    html! {
        <div class={ "guess" } style={ guess_css }>
            <div class={ "neutral thumbnail" } style={ fade_in(0.0) }>
                <img class={"thumb-img"} src={ thumb_url } />
                <span class={"title"}>{ title }</span>
            </div>
            <div class={ campaign_cls } style={ fade_in(animation_delay) }>{ campaign }</div>
            <div class={ dur_cls } style={ fade_in(2.0 * animation_delay) }>{ dur }</div>
            <div class={ date_cls } style={ fade_in(3.0 * animation_delay) }>{ date }</div>
            <div class={ player_count_cls } style={ fade_in(4.0 * animation_delay) }>{ player_count } { " jogadores" }</div>
            <div class={ cinematic_cls } style={ fade_in(5.0 * animation_delay) }>{ has_cinematic }</div>
        </div>
    }
}

#[function_component(Titles)]
fn titles() -> Html {
    html! {
        <div class={ "titles-wrapper" }>
            <div class={ "titles" }>
                <div class={ "neutral" }>{ "Título" }</div>
                // <div class={ "neutral" }>{ "Nro do Episódio" }</div>
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

    let today_idx = (413 * 413) % episodes.len();
    let today_ep = &episodes[today_idx].clone();

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
            <input oninput={on_input.clone()} list={ "episodes" } type={ "text" } placeholder={ "Adivinhe o episódio..." } id="episode-guess" style="width:895px;" />
            <datalist id="episodes"> { options } </datalist>
            < Titles/ >
            <ul id="guesses"> {
                list
            } </ul>
        </>
    }
}
