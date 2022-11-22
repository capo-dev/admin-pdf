use admin::CourtCase;
use chrono::NaiveDate;
use serde::Deserialize;

use crate::error::Error;

#[derive(Deserialize)]
pub struct CSVCourtCase {
    #[serde(rename(deserialize = "Case Number"))]
    case_number: String,
    #[serde(
        rename(deserialize = "Next Hearing Date"),
        with = "middle_endian_format"
    )]
    hearing_date: Option<NaiveDate>,
}

mod middle_endian_format {
    use admin::MIDDLE_ENDIAN_FORMAT;
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)
            .ok()
            .and_then(|s| NaiveDate::parse_from_str(&s, MIDDLE_ENDIAN_FORMAT).ok()))
    }
}

impl CSVCourtCase {
    pub fn into_court_case(self, default_hearing_date: NaiveDate) -> Result<CourtCase, Error> {
        Ok(CourtCase {
            case_number: self.case_number,
            hearing_date: self.hearing_date.unwrap_or(default_hearing_date),
        })
    }
}
