use web_sys::HtmlInputElement;
use yew::{Callback, InputEvent, KeyboardEvent, TargetCast, UseStateHandle};

use crate::episode::Episode;

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
    correct_ep: Episode,
    guesses_state: UseStateHandle<Vec<Episode>>,
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
            g.push(ep.clone());
            guesses.set(g);

            if guess == correct.title {
                has_guessed.set(true);
            }
        }
    })
}
