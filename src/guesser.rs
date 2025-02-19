use chrono::Datelike;
use crate::{episode::Episode, theme};
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
        true => format!("
            background-color: {0}ee;
            -webkit-box-shadow:0px 0px 5px 3px {0}cc;
            -moz-box-shadow: 0px 0px 5px 3px {0}cc;
            box-shadow: 0px 0px 5px 3px {0}cc;
        ", theme),
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
    
    let thumb_url = format!("res/{}", guess.episode.cover_path);
    let title = format!("{} - {}", guess.episode.number, guess.episode.title);
    let campaign = guess.episode.campaign.as_str();
    let dur = guess.episode.dur_fmt();
    let date = guess.episode.date.year().to_string();
    let player_count = guess.episode.players - 1;
    
    let fade_in = |dur: f64| format!("animation-delay: {}s;", dur);
    
    html! {
        <div class={ "guess" } style={ guess_css }>
            <div class={ "neutral thumbnail" } style={ fade_in(0f64) }>
                <img class={"thumb-img"} src={ thumb_url } />
                <span class={"title"}>{ title }</span>
            </div>
            <div class={ campaign_cls } style={ fade_in(0.5f64) }>{ campaign }</div>
            <div class={ dur_cls } style={ fade_in(1f64) }>{ dur }</div>
            <div class={ date_cls } style={ fade_in(1.5f64) }>{ date }</div>
            <div class={ player_count_cls } style={ fade_in(2f64) }>{ player_count } { " jogadores" }</div>
        </div>
    }
}

#[function_component(Guesser)]
pub fn guesser() -> Html {
    const EPISODES: &str = include_str!("../res/data/episodes.json");
    
    let guesses: UseStateHandle<Vec<Episode>> = use_state(Vec::new);
    let episodes: Vec<Episode> = serde_json::from_str(EPISODES).unwrap();
    
    let today_idx = 413413413 % episodes.len();
    let today_ep = &episodes[today_idx % episodes.len()].clone();
    
    let on_input = Callback::from({
        let episodes = episodes.clone();
        let guesses = guesses.clone();
        let correct = today_ep.clone();
        
        move |e: InputEvent| {
            let Some(guess) = e.data() else { return; };
            if guesses.iter().any(|ep| ep == &correct) { return; }
            if guesses.iter().any(|ep| ep.title == guess) { return; }
            
            let Some(ep) = episodes.iter().find(|ep| ep.title == guess) else { return; };
            
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
            <ul id="guesses"> { 
                list
            } </ul>
        </>
    }
}
