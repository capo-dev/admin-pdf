use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    Reqwest {
        url: Option<String>,
        message: String,
    },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourtCase {
    pub case_number: String,
    pub hearing_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HarrisQuery {
    pub court: u16,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub const MIDDLE_ENDIAN_FORMAT: &'static str = "%m/%d/%Y";
