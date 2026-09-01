use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct HourlyRaw {
    pub time: Vec<Option<String>>,
    pub pm2_5: Vec<Option<f64>>,
    pub pm10: Vec<Option<f64>>,
}