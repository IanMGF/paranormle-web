use chrono::Datelike;

pub enum Element {
    Blood,
    Death,
    Knowledge,
    Energy,
    Fear,
}

pub fn get_day_bg() -> String {
    let now = chrono::Local::now();
    let day = now.day() as usize;
    
    const ELEMENTS: [&str; 5] = ["Sangue", "Morte", "Conhecimento", "Energia", "Medo"];

    let element_idx = day % ELEMENTS.len();
    format!("{}.jpeg", ELEMENTS[element_idx])
}

pub fn get_day_guess_color() -> String {
    let now = chrono::Local::now();
    let day = now.day() as usize;
    
    const COLORS: [&str; 5] = ["#9c091d", "#000000", "#bdb55b", "#b43cba", "#4848c2"];

    let color_idx = day % COLORS.len();
    String::from(COLORS[color_idx])
}