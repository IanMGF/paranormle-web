use paranormle::guesser::Guesser;
use paranormle::reset_counter::ResetCountdown;
use paranormle::theme::Theme;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let bg_img = Theme::gen_day_element().get_bg();
    let bg_style = format!("background-image: url(\'res/backgrounds/{}\');", bg_img);

    html! {
        <div id={ "container" } style={ bg_style }>
            <h1>{ "Paranormle" }</h1>
            <ResetCountdown />
            <Guesser />

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
