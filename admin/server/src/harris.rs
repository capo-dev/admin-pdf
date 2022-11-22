use crate::error::Error;
use actix_web::{get, web, HttpResponse};
use admin::{HarrisQuery, MIDDLE_ENDIAN_FORMAT};
use chrono::NaiveDate;

use crate::csv_court_case::CSVCourtCase;

#[get("/harris")]
pub async fn harris(query: web::Query<HarrisQuery>) -> Result<HttpResponse, Error> {
    let client = reqwest::ClientBuilder::new().build()?;

    let court = query.0.court.to_string();
    let start_date = query.0.start_date.format(MIDDLE_ENDIAN_FORMAT).to_string();
    let end_date = query.0.end_date.format(MIDDLE_ENDIAN_FORMAT).to_string();
    let url = "https://jpwebsite.harriscountytx.gov/PublicExtracts/GetExtractData";
    // ?extractCaseType=CV&extract=6&court={court}&casetype=EV&format=csv&fdate={startDate}&tdate={endDate}";
    let qs = [
        ("extractCaseType", "CV"),
        ("extract", "6"),
        ("casetype", "EV"),
        ("format", "csv"),
        ("court", court.as_str()),
        ("fdate", start_date.as_str()),
        ("tdate", end_date.as_str()),
    ];

    let req = client.get(url).query(qs.as_ref()).build()?;
    let res = client
        .execute(req)
        .await?
        .error_for_status()?
        .text()
        .await?;

    let mut rdr = csv::Reader::from_reader(res.as_bytes());
    for record in rdr.deserialize() {
        let csv_court_case: CSVCourtCase = match record {
            Ok(csv_eviction) => csv_eviction,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        let court_case = match csv_court_case.into_court_case(NaiveDate::default()) {
            Ok(court_case) => court_case,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        println!("{:?}", court_case);
    }

    Ok(HttpResponse::Ok().body(res))
}
