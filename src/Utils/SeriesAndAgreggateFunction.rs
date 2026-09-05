use std::collections::HashMap;
use time::{Duration, OffsetDateTime};
use crate::enums::PointKind::PointKind;
use crate::enums::Pollutant::Pollutant;
use crate::enums::Unit::Unit;
use crate::errors::GeneralError::GeneralError;
use crate::model::dto::Reading::Reading;
use crate::model::Point::Point;
use crate::model::Series::Series;
use crate::utils::FoundStepByReadingData::found_step_by_reading_data;
use crate::utils::MountPointData::mount_point_data;
use crate::utils::MountSeries::mount_series;

pub fn to_series(leituras: &[Reading], instant_now: OffsetDateTime) -> Result<HashMap<Pollutant, Series>, GeneralError>{

    let mut it:usize = 1;
    let mut pol_pm25:Vec<Reading> = Vec::new();
    let mut pol_pm10:Vec<Reading> = Vec::new();

    let mut mapa: HashMap<Pollutant, Series> = HashMap::new();
    let mut modal_pm25:HashMap<String, i64>   = HashMap::new();
    let mut modal_pm10:HashMap<String, i64>   = HashMap::new();

    for i in leituras {
        match i.pollutant {
            Pollutant::Pm25 => {
                let diff = leituras.get(it).unwrap().at - i.at;
                modal_pm25.insert(format!("{} - {}", i.at, leituras.get(it).unwrap().at), (diff.whole_seconds() / 60) / 60 );
                pol_pm25.push(i.clone());
            }
            Pollutant::Pm10 => {
                let diff = leituras.get(it).unwrap().at - i.at;
                modal_pm10.insert(format!("{} - {}", i.at, leituras.get(it).unwrap().at), (diff.whole_seconds() / 60) / 60 );
                pol_pm10.push(i.clone());
            }
        }
        if it < leituras.len() - 1 {
            it += 1;
        }
    }

    mount_series(&mut mapa, mount_point_data(&pol_pm25, &instant_now), found_step_by_reading_data(&modal_pm25), Pollutant::Pm25);
    mount_series(&mut mapa, mount_point_data(&pol_pm10, &instant_now), found_step_by_reading_data(&modal_pm10), Pollutant::Pm10);

    Ok(mapa)

}