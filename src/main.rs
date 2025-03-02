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

#[function_component(Head)]
fn head() -> Html {
    html! {
        <>
            <title>{ "Paranormle" }</title>
            <meta charset={ "utf-8" } />
            <meta name={ "description" } content={ "Adivinhe o episódio de Ordem Paranormal!" } />
            <meta name={ "author" } content={ "Ian M. G. Freitas" } />
            <meta name={ "viewport" } content={ "width=device-width, initial-scale=1.0" } />

            <link rel="stylesheet" type="text/css" href="res/style.css" />
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin={ "true" } />
            <link href={ "https://fonts.googleapis.com/css2?family=Girassol&display=swap" } rel="stylesheet" />
        < />
    }
}

fn main() {
    let head = gloo::utils::head();

    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
    yew::Renderer::<Head>::with_root(head.into()).render();
}
