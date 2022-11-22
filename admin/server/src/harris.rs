use actix_web::{get, HttpResponse};
use crate::error::Error;


#[get("/harris")]
pub async fn harris() -> Result<HttpResponse, Error> {
    let client = reqwest::ClientBuilder::new().build()?;

    let court = "350";
    let start_date = "11/01/2022";
    let end_date = "11/15/2022";
    let url = "https://jpwebsite.harriscountytx.gov/PublicExtracts/GetExtractData";
    // ?extractCaseType=CV&extract=6&court={court}&casetype=EV&format=csv&fdate={startDate}&tdate={endDate}";
    let qs = [
        ("extractCaseType", "CV"),
        ("extract", "6"),
        ("casetype", "EV"),
        ("format", "csv"),
        ("court", court),
        ("fdate", start_date),
        ("tdate", end_date),
    ];

    let req = client.get(url).query(qs.as_ref()).build()?;
    let res = client.execute(req).await?.error_for_status()?.text().await?;

    Ok(HttpResponse::Ok().body(res))
}
