use std::fmt::format;
use std::time::Duration;
use reqwest::blocking::Client;
use reqwest::Url;
use crate::errors::GeneralError::GeneralError;
use crate::model::AirQualityRaw::AirQualityRaw;

pub fn http_client(lat: f32, lon: f32) -> Result<AirQualityRaw, GeneralError>{

    let mut json_content = AirQualityRaw::default();

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(3))
        .build().unwrap();

    let mut url = Url::parse("https://air-quality-api.open-meteo.com").unwrap();
    url.path_segments_mut()
        .unwrap().clear()
        .push("v1")
        .push("air-quality");

    url.query_pairs_mut().clear()
        .append_pair("latitude", lat.to_string().as_str())
        .append_pair("longitude", lon.to_string().as_str())
        .append_pair("hourly", "pm2_5,pm10,ozone")
        .append_pair("timezone", "auto")
        .append_pair("forecast_days", "1")
        .append_pair("past_days", "1");

    match client.get(url).send() {
        Ok(response) => {

            if response.status().is_client_error() {
                return Err(GeneralError::FalhaProcessarRequisicaoHTTP(response.text().unwrap()));
            }

            if response.status().is_success(){
                let r = serde_json::from_str::<AirQualityRaw>(&response.text().unwrap());
                if let Ok(r) = r {
                    json_content = r;
                }
                
            }

        }
        Err(e) => {

            if e.is_timeout() {
                return Err(GeneralError::FalhaProcessarRequisicaoHTTP("timeout".to_string()));
            }

        }
    }

    Ok(json_content)

}
