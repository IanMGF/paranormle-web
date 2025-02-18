use paranormle::episode::Episode;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Eq)]
struct GuessProp {
    episode: Episode,
    correct: Episode,
}

#[function_component(Guess)]
fn guess(guess: &GuessProp) -> Html {
    let campaign_cls = match guess.episode.campaign == guess.correct.campaign {
        true => "correct",
        false => "incorrect",
    };
    
    let date_cls = match Ord::cmp(&guess.episode.date, &guess.correct.date) {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    };
    
    let dur_cls = match Ord::cmp(&guess.episode.duration, &guess.correct.duration) {
        std::cmp::Ordering::Equal => "correct",
        std::cmp::Ordering::Less => "less",
        std::cmp::Ordering::Greater => "greater",
    };
    
    html! {
        <div class="guess">
            <div class={ "neutral thumbnail" }>
            <img class={"thumb-img"} src={ String::from("res/") + guess.episode.cover_path.as_str() } />
                <span class={"title"}>{guess.episode.number} { " - " } { guess.episode.title.as_str() }</span>
            </div>
            <div class={ campaign_cls }>{ guess.episode.campaign.as_str() }</div>
            <div class={ dur_cls }>{ guess.episode.dur_fmt() }</div>
            <div class={ date_cls }>{ guess.episode.date.format("%d/%m/%Y").to_string() }</div>
        </div>
    }
}

#[function_component(Guesser)]
fn guesser() -> Html {
    const EPISODES: &str = include_str!("../res/data/episodes.json");
        
    let guesses: UseStateHandle<Vec<Episode>> = use_state(Vec::new);
    let episodes: Vec<Episode> = serde_json::from_str(EPISODES).unwrap();
    
    // let today_idx = rand::random::<u32>() % (tracks.len() as u32);
    let today_idx = 30;
    let today_ep = &episodes[today_idx as usize];
    
    let on_input = {
        Callback::from({
            let episodes = episodes.clone();
            let guesses = guesses.clone();
            
            move |e: InputEvent| {
                let Some(guess) = e.data() else { return; };
                
                if let Some(track) = episodes.iter().find(|ep| ep.title == guess) {
                    let mut g = guesses.to_vec();
                    g.push(track.clone());
                    guesses.set(g);
                }
            }
        })
    };
    
    html! {
        <>
            <input oninput={on_input.clone()} list={ "tracks" } type={ "text" } placeholder={ "Adivinhe o episódio..." } id="track-guess" style="width:30%;" />
            <datalist id="tracks">
                { for episodes.iter().map(|ep| html! { <option value={ ep.title.clone() } /> }) }
            </datalist>
            
            <ul id="guesses">
                { for guesses.iter().rev().map(|ep| html! { 
                    <li class="guess-wrapper">
                        <Guess episode={ ep.clone() } correct={ today_ep.clone() }></Guess>
                    </li> 
                }) }
            </ul>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    let bg_img = paranormle::backgrond::get_day_bg();
    html! {
        <div id={ "container" } style={ format!("background-image: url(\'res/backgrounds/{}\');", bg_img) }>
            <h1>{ "Paranormle" }</h1>
            <Guesser />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}