use chrono::Datelike;
use std::cmp::Ordering;
use yew::{function_component, html, Html, Properties};

use crate::{episode::Episode, theme::Theme};

use super::info_piece::InfoPiece;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct GuessProp {
    pub episode: Episode,
    pub correct: Episode,
}

#[function_component(Guess)]
pub fn guess(guess: &GuessProp) -> Html {
    let theme = Theme::gen_day_element();
    let guess_css = match guess.episode == guess.correct {
        true => theme.correct_guess_css(),
        false => String::from("background-color: #17171777;"),
    };

    let guess_result = EpisodeGuessResult::from_guess(&guess.episode, &guess.correct);

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

    let animation_delay = 3.13 / 6.0;
    let fade_in = |step: f64| format!("animation-delay: {}s;", step * animation_delay);

    html! {
        <div class={ "guess" } style={ guess_css }>
            <div class={ "neutral thumbnail" } style={ fade_in(0.0) }>
                <img class={"thumb-img"} src={ thumb_url } />
                <span class={"title"}>{ title }</span>
            </div>
            <div class={ guess_result.number.get_css() } style={ fade_in(1.0) }>{ ep_number }</div>
            <div class={ guess_result.campaign.get_css() } style={ fade_in(2.0) }>{ campaign }</div>
            <div class={ guess_result.duration.get_css() } style={ fade_in(3.0) }>{ dur }</div>
            <div class={ guess_result.year.get_css() } style={ fade_in(4.0) }>{ date }</div>
            <div class={ guess_result.players.get_css() } style={ fade_in(5.0) }>{ player_count } { " jogadores" }</div>
            <div class={ guess_result.cinematic.get_css() } style={ fade_in(6.0) }>{ has_cinematic }</div>
        </div>
    }
}

pub struct EpisodeGuessResult {
    pub number: InfoPiece<u32, Ordering>,
    pub campaign: InfoPiece<String, bool>,
    pub duration: InfoPiece<u64, Ordering>,
    pub year: InfoPiece<i32, Ordering>,
    pub players: InfoPiece<u8, Ordering>,
    pub cinematic: InfoPiece<bool, bool>,
}

impl EpisodeGuessResult {
    pub fn from_guess(guess: &Episode, correct: &Episode) -> Self {
        Self {
            number: InfoPiece::from_comparison(&guess.number, &correct.number),
            campaign: InfoPiece::from_comparison(&guess.campaign, &correct.campaign),
            duration: InfoPiece::from_comparison(&guess.duration, &correct.duration),
            year: InfoPiece::from_comparison(&guess.date.year(), &correct.date.year()),
            players: InfoPiece::from_comparison(&guess.players, &correct.players),
            cinematic: InfoPiece::from_comparison(&guess.has_cinematic, &correct.has_cinematic),
        }
    }
}
