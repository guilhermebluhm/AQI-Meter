use std::collections::HashMap;
use time::Duration;
use crate::enums::Pollutant::Pollutant;
use crate::enums::Unit::Unit;
use crate::model::Point::Point;
use crate::model::Series::Series;
use crate::utils::FoundStepByReadingData::found_step_by_reading_data;

pub fn mount_series(mapa_serie: &mut HashMap<Pollutant, Series>, pontos: Vec<Point>, step: i64, tipo_poluente: Pollutant) {

    let mut ser = Series{
        polluent: Pollutant::Pm25,
        unit: Unit::MicrogramsPerCubicMeter,
        points: pontos,
        step: Duration::hours(step)
    };

    if tipo_poluente == Pollutant::Pm10 {
        ser.polluent = Pollutant::Pm10;
        mapa_serie.insert(Pollutant::Pm10, ser);
    }
    else{
        mapa_serie.insert(Pollutant::Pm25, ser);
    }

}
