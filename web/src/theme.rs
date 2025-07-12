use chrono::{NaiveDate, TimeDelta};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
    Blood,
    Death,
    Knowledge,
    Energy,
    Fear,
    DeathKnowledge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme(Element);

impl Theme {
    pub fn gen_day_element() -> Self {
        let day_zero: NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
        let curr_date = chrono::Local::now().date_naive();

        if curr_date == NaiveDate::from_ymd_opt(2025, 7, 12).unwrap() {
            return Theme(Element::DeathKnowledge);
        }

        let day_count = curr_date.signed_duration_since(day_zero).num_days() as usize;

        const ELEMENTS: [Element; 5] = [
            Element::Blood,
            Element::Death,
            Element::Knowledge,
            Element::Energy,
            Element::Fear,
        ];

        let element_idx = day_count % ELEMENTS.len();
        let element = ELEMENTS[element_idx];
        Theme(element)
    }

    pub fn get_bg(&self) -> &str {
        match self.0 {
            Element::Blood => "Sangue.jpeg",
            Element::Death => "Morte.jpeg",
            Element::Knowledge => "Conhecimento.jpeg",
            Element::Energy => "Energia.jpeg",
            Element::Fear => "Medo.jpeg",
            Element::DeathKnowledge => "Conhecimento+Morte.jpeg",
        }
    }

    const fn get_guess_color(&self) -> &'static str {
        match self.0 {
            Element::Blood => "#9c091d",
            Element::Death => "#000000",
            Element::Knowledge => "#bdb55b",
            Element::Energy => "#b43cba",
            Element::Fear => "#bdbdbd",
            Element::DeathKnowledge => "#96914e",
        }
    }

    pub fn correct_guess_css(&self) -> String {
        let solid_color = self.get_guess_color();
        let bg_color = format!("{}ee;", solid_color);
        let shadow_color = format!("{}cc;", solid_color);

        format!(
            "
            background-color: {0};
            -webkit-box-shadow:0px 0px 5px 3px {1};
            -moz-box-shadow: 0px 0px 5px 3px {1};
            box-shadow: 0px 0px 5px 3px {1};
        ",
            bg_color, shadow_color
        )
    }
}
