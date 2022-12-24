use teloxide::RequestError;

use crate::evedge_api::EvedgeChargingStation;

pub async fn fetch_station_by_id(station_id: &str) -> Result<EvedgeChargingStation, RequestError> {
    let url = format!("https://user.evedge.co.il/DuskyWebApi//noauthlocation?Id={}&isOldApi=false&UiCulture=he-IL", station_id);
    let response = reqwest::get(url).await?;
    let json_content = response.text().await?;

    serde_json::from_str(&json_content).map_err(|err| RequestError::InvalidJson {
        source: err,
        raw: json_content.into_boxed_str(),
    })
}
