use chrono::Datelike;


pub fn get_day_bg() -> String {
    let now = chrono::Local::now();
    let day = now.day() as usize;
    
    const ELEMENTS: [&str; 5] = ["Sangue", "Morte", "Conhecimento", "Energia", "Medo"];

    let element_idx = day % ELEMENTS.len();
    format!("{}.jpeg", ELEMENTS[element_idx])
}