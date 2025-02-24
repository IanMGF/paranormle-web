use chrono::{NaiveTime, TimeDelta};
use gloo_timers::callback::Interval;
use yew::{function_component, html, use_state, Html};

fn get_formatted_countdown() -> String {
    let next_day = chrono::Local::now().date_naive() + chrono::Duration::days(1);
    let now = chrono::Local::now();
    let until_next_day: TimeDelta = next_day.and_time(NaiveTime::default()) - now.naive_local();

    format!(
        "{:02}:{:02}:{:02}",
        until_next_day.num_hours(),
        until_next_day.num_minutes() % 60,
        until_next_day.num_seconds() % 60
    )
}

#[function_component(ResetCountdown)]
pub fn day_countdown() -> Html {
    let formatted = get_formatted_countdown();
    let time_state = use_state(|| formatted);

    let interval = Interval::new(1000u32, {
        let time_state = time_state.clone();
        move || {
            let formatted = get_formatted_countdown();
            time_state.set(formatted);
        }
    });

    // Start the interval
    interval.forget();

    html! {
        <div id="reset-countdown">
            <h2>{ "Tempo para o próximo episódio:" }</h2>
            <h3>{ (*time_state).clone() }</h3>
        </div>
    }
}
