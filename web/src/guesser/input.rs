use std::rc::Rc;

use stylist::style;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, Callback, Html, InputEvent, KeyboardEvent, Properties, TargetCast,
    UseStateHandle,
};

use common::episode::Episode;

trait GuessEvent {
    fn get_guess(&self) -> Option<String>;
}

impl GuessEvent for KeyboardEvent {
    fn get_guess(&self) -> Option<String> {
        if self.key() == "Enter" {
            let input: HtmlInputElement = self.target_unchecked_into();
            Some(input.value())
        } else {
            None
        }
    }
}

impl GuessEvent for InputEvent {
    fn get_guess(&self) -> Option<String> {
        self.data()
    }
}

#[allow(private_bounds)]
pub fn guess_callback<T: GuessEvent>(
    episodes: &[Episode],
    correct_ep: Rc<Episode>,
    guesses_state: UseStateHandle<Vec<Rc<Episode>>>,
    has_guessed: UseStateHandle<bool>,
) -> Callback<T> {
    Callback::from({
        let episodes = Vec::from(episodes);
        let guesses = guesses_state.clone();
        let correct = correct_ep.clone();

        move |e: T| {
            let Some(guess) = e.get_guess() else {
                return;
            };

            if guesses.iter().any(|ep| ep.title == guess) {
                return;
            }
            let Some(ep) = episodes.iter().find(|ep| ep.title == guess) else {
                return;
            };

            let mut g = guesses.to_vec();
            g.push(Rc::new(ep.clone()));
            guesses.set(g);

            if guess == correct.title {
                has_guessed.set(true);
            }
        }
    })
}

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    pub episode_list: Rc<Vec<Episode>>,
    pub episode_of_the_day: Rc<Episode>,
    pub guesses: UseStateHandle<Vec<Rc<Episode>>>,
    pub has_guessed: UseStateHandle<bool>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_style = style!(
        "
        width: 895px;
        height: 30px;
    "
    )
    .expect("Failed to create Guesser input style");

    let input_callback = guess_callback(
        &props.episode_list,
        props.episode_of_the_day.clone(),
        props.guesses.clone(),
        props.has_guessed.clone(),
    );
    let event_callback = guess_callback(
        &props.episode_list,
        props.episode_of_the_day.clone(),
        props.guesses.clone(),
        props.has_guessed.clone(),
    );

    let options: Vec<&Episode> = props
        .episode_list
        .iter()
        .filter(|&ep| props.guesses.iter().cloned().all(|guess| *guess != *ep))
        .collect::<Vec<&Episode>>();

    let options_html = options
        .iter()
        .map(|ep| html! { <option value={ ep.title.clone() }></option> })
        .collect::<Html>();
    html! {
        <>
            < datalist id="episodes" > { options_html } </datalist>
            <input
                onkeyup={ input_callback }
                oninput={ event_callback }
                list={ "episodes" }
                type={ "text" }
                placeholder={ "Adivinhe o episódio..." }
                id="episode-guess"
                class={ input_style.get_class_name().to_owned() + " centered" }
                disabled={ *props.has_guessed }
            />
        </>
    }
}
