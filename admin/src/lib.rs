use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CourtCase {
    case_number: String,
    hearing_date: NaiveDate,
}
