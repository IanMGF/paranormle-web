use chrono::NaiveDate;

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
        let day_zero: NaiveDate = chrono::NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();
        let curr_date = chrono::Local::now().date_naive();

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

    pub fn get_bg(&self) -> String {
        match self.0 {
            Element::Blood => String::from("Sangue.jpeg"),
            Element::Death => String::from("Morte.jpeg"),
            Element::Knowledge => String::from("Conhecimento.jpeg"),
            Element::Energy => String::from("Energia.jpeg"),
            Element::Fear => String::from("Medo.jpeg"),
        }
    }

    const fn get_guess_color(&self) -> &'static str {
        match self.0 {
            Element::Blood => "#9c091d",
            Element::Death => "#000000",
            Element::Knowledge => "#bdb55b",
            Element::Energy => "#b43cba",
            Element::Fear => "#bdbdbd",
        }
    }

    pub fn correct_guess_css(&self) -> String {
        format!(
            "
            background-color: {0}ee;
            -webkit-box-shadow:0px 0px 5px 3px {0}cc;
            -moz-box-shadow: 0px 0px 5px 3px {0}cc;
            box-shadow: 0px 0px 5px 3px {0}cc;
        ",
            self.get_guess_color()
        )
    }
}
