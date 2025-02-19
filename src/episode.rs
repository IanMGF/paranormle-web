use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub campaign: String,
    #[serde(with = "date_format")]
    pub date: NaiveDate,
    pub duration: u64,
    pub cover_url: String,
    pub cover_path: String,
    pub number: u32,
    pub players: u8,
}

impl Episode {
    pub fn dur_fmt(&self) -> String {
        let dur_hr = self.duration / 3600;
        let dur_min = (self.duration % 3600) / 60;
        let dur_sec = self.duration % 60;
        format!("{}:{:02}:{:02}", dur_hr, dur_min, dur_sec)
    }
}

mod date_format {
    use chrono::{Datelike, NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%d/%m/%Y";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).ok_or_else(|| serde::de::Error::custom("invalid date"))
    }
}