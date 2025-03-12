use paranormle::guesser::Guesser;
use paranormle::reset_countdown::ResetCountdown;
use paranormle::theme::Theme;
use stylist::{style, Style};
use web_sys::Element;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let bg_img = Theme::gen_day_element().get_bg();

    let body_style: Style = style!("
        background-attachment: fixed;
        background-position: center;
        background-size: 100%;
        background-image: url(\"res/backgrounds/${bg_img}\");
        width: 100vw;
        height: 100vh;
        overflow-x: hidden;
        overflow-y: auto;
        margin: 0 0 0 0;
    ", bg_img=bg_img).expect("Failed to create container class");
    
    html! {
        <div class={ body_style.get_class_name().to_owned() }>
            <h1 class="centered">{ "Paranormle" }</h1>
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
    wasm_logger::init(wasm_logger::Config::default());

    let head: Element = gloo::utils::head().into();

    yew::Renderer::<App>::new().render();
    yew::Renderer::<Head>::with_root(head).render();
}
