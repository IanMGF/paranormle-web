use chrono::{NaiveDate, NaiveTime, TimeDelta};
use chrono_tz::America::Sao_Paulo;
use gloo_timers::callback::Interval;
use yew::{function_component, html, use_state, Html, UseStateHandle};

fn time_until_next_day(today: NaiveDate) -> TimeDelta {
    let now = chrono::Local::now();
    let next_day = {
        let tomorrow = today + chrono::Duration::days(1);
        tomorrow.and_time(NaiveTime::default())
    };

    next_day - now.naive_local()
}

#[function_component(ResetCountdown)]
pub fn day_countdown() -> Html {
    let is_counting: UseStateHandle<bool> = use_state(|| true);
    let today: UseStateHandle<NaiveDate> = use_state(|| {
        let now = chrono::Local::now();
        now.with_timezone(&Sao_Paulo).date_naive()
    });
    
    let interval = Interval::new(1000u32, {
        let is_counting = is_counting.clone();
        let today = today.clone();
        move || {
            if !(*is_counting) { return; }
            
            let until_next_day = time_until_next_day(*today);
            
            let new_time = format!(
                "{:02}:{:02}:{:02}",
                until_next_day.num_hours(),
                until_next_day.num_minutes() % 60,
                until_next_day.num_seconds() % 60,  
            );
            
            if until_next_day < TimeDelta::microseconds(0) {
                gloo::utils::document()
                    .get_element_by_id("countdown")
                    .expect("Element with id 'countdown' not found")
                    .set_text_content(Some("A resposta reiniciará quando atualizar a página"));
                is_counting.set(false);
            } else {
                gloo::utils::document()
                    .get_element_by_id("countdown")
                    .expect("Element with id 'countdown' not found")
                    .set_text_content(Some(new_time.as_str()));
            }
        }
    });

    // Start the interval
    interval.forget();
    
    let until_next_day = time_until_next_day(*today);
    html! {
        <div id={ "reset-countdown" } class={ "centered" }>
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
