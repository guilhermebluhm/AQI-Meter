use std::collections::HashMap;
use std::iter::Map;
use serde::Deserialize;
use crate::model::HourlyRaw::HourlyRaw;

#[derive(Debug, Deserialize, Default)]
pub struct AirQualityRaw{
    pub latitude: f64,
    pub longitude: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub hourly_units: HashMap<String, String>, //talvez nao seja necessario este atributo (em analise)
    pub hourly: HourlyRaw,
}