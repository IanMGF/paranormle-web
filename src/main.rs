use paranormle::guesser::Guesser;
use paranormle::mist::Mist;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let bg_img = paranormle::theme::get_day_bg();
    html! {
        <div id={ "container" } style={ format!("background-image: url(\'res/backgrounds/{}\');", bg_img) }>
            <h1>{ "Paranormle" }</h1>
            <Guesser />
            <Mist />

            <span id="creditos-wpp">
                <a href="https://x.com/desconjurado/status/1543281129385594881">
                    { "Wallpapers por Sumo (@desconjurado)" }
                </a>
            </span>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
