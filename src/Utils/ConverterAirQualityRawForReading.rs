use time::OffsetDateTime;
use crate::enums::Pollutant::Pollutant;
use crate::enums::Unit::Unit;
use crate::errors::GeneralError::GeneralError;
use crate::model::AirQualityRaw::AirQualityRaw;
use crate::model::dto::Concentration::Concentration;
use crate::model::dto::Reading::Reading;
use crate::utils::ConverterInputTimeForOffsetDateTime::converter_input_date;

pub fn to_reading(air_quality: &AirQualityRaw) -> Result<Vec<Reading>, GeneralError>{

    let arrays = &air_quality.hourly;
    let default_arr_size = air_quality.hourly.time.len();
    let mut vec_reading: Vec<Reading> = Vec::with_capacity(default_arr_size);

    if default_arr_size > arrays.pm2_5.len() || default_arr_size > arrays.pm10.len() {
        return Err(GeneralError::FalhaAoMontarTipo("Incorrect array size".to_string()))
    }

    for ((t, pm25), pm10) in
        arrays.time.iter().zip(arrays.pm2_5.iter()).zip(arrays.pm10.iter()){

        if let Some(x) = pm25{
            let concentration = Concentration::new(*x, Unit::MicrogramsPerCubicMeter)?;
            let reading = Reading{
                pollutant: Pollutant::Pm25,
                value: concentration,
                at: converter_input_date(t.clone().unwrap().as_str(), air_quality.utc_offset_seconds as i64)?
            };
            vec_reading.push(reading);
        }

        if let Some(x) = pm10{
            let concentration = Concentration::new(*x, Unit::MicrogramsPerCubicMeter)?;
            let reading = Reading{
                pollutant: Pollutant::Pm10,
                value: concentration,
                at: converter_input_date(t.clone().unwrap().as_str(), air_quality.utc_offset_seconds as i64)?
            };
            vec_reading.push(reading);
        }

    }
    Ok(vec_reading)
}

pub fn at_hour(readings: &Vec<Reading>, data_offset: OffsetDateTime) -> Vec<Reading>{

    let mut vec_reading: Vec<Reading> = vec![];

    for i in readings{

        if i.at.eq(&data_offset) {
            vec_reading.push(*i);
        }

    }
    
    vec_reading

}