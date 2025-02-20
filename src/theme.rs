use chrono::Datelike;

#[derive(Clone, Copy, Debug)]
pub enum Element {
    Blood,
    Death,
    Knowledge,
    Energy,
    Fear,
}

pub struct Theme(Element);

impl Theme {
    pub fn gen_day_element() -> Self {
        let now = chrono::Local::now();
        let day = now.day() as usize;

        const ELEMENTS: [Element; 5] = [
            Element::Blood,
            Element::Death,
            Element::Knowledge,
            Element::Energy,
            Element::Fear,
        ];

        let element_idx = day % ELEMENTS.len();
        let element = ELEMENTS[element_idx];
        Theme(element)
    }

    pub fn get_bg(&self) -> String {
        match self.0 {
            Element::Blood => String::from("Sangue.jpeg"),
            Element::Death => String::from("Morte.jpeg"),
            Element::Knowledge => String::from("Conhecimento.jpeg"),
            Element::Energy => String::from("Energia.jpeg"),
            Element::Fear => String::from("Medo.jpeg"),
        }
    }

    pub fn get_guess_color(&self) -> String {
        match self.0 {
            Element::Blood => String::from("#9c091d"),
            Element::Death => String::from("#000000"),
            Element::Knowledge => String::from("#bdb55b"),
            Element::Energy => String::from("#b43cba"),
            Element::Fear => String::from("#bdbdbd"),
        }
    }
}
