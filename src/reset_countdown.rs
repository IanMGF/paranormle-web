use chrono::{NaiveTime, TimeDelta};
use gloo_timers::callback::Interval;
use yew::{function_component, html, Html};

#[function_component(ResetCountdown)]
pub fn day_countdown() -> Html {
    let mut until_next_day: TimeDelta = {
        let now = chrono::Local::now();
        let next_day = chrono::Local::now().date_naive() + chrono::Duration::days(1);
        let next_day = next_day.and_time(NaiveTime::default());

        next_day - now.naive_local()
    };

    let interval = Interval::new(1000u32, move || {
        until_next_day -= chrono::Duration::seconds(1);
        let new_time = format!(
            "{:02}:{:02}:{:02}",
            until_next_day.num_hours(),
            until_next_day.num_minutes() % 60,
            until_next_day.num_seconds() % 60
        );

        gloo::utils::document()
            .get_element_by_id("countdown")
            .expect("Element with id 'countdown' not found")
            .set_text_content(Some(new_time.as_str()));
    });

    // Start the interval
    interval.forget();

    html! {
        <div id="reset-countdown" class="centered">
            <h2>{ "Tempo para o próximo episódio:" }</h2>
            <h3 id={ "countdown" }>{
                format!(
                    "{:02}:{:02}:{:02}",
                    until_next_day.num_hours(),
                    until_next_day.num_minutes() % 60,
                    until_next_day.num_seconds() % 60
                )
            }</h3>
        </div>
    }
}
