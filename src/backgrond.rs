use chrono::Datelike;


pub fn get_day_bg() -> String {
    let now = chrono::Local::now();
    let day = now.day();
    
    const ELEMENTS: [&str; 4] = ["Sangue", "Morte", "Conhecimento", "Energia"];

    let element_idx = day % 4;
    format!("{}.jpeg", ELEMENTS[element_idx as usize])
}