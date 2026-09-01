use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};
use crate::errors::GeneralError::GeneralError;

pub fn converter_input_date(value: &str, offset_seconds: i64) -> Result<OffsetDateTime, GeneralError>{

    let format = format_description!("[year]-[month]-[day]T[hour]:[minute]");
    let time_with_offset_format = PrimitiveDateTime::parse(value, &format)
        .map_err(|e| GeneralError::FalhaAoMontarTipo(e.to_string()))?;
    let offset_for_composition_time = UtcOffset::from_whole_seconds(offset_seconds as i32)
        .map_err(|e| GeneralError::FalhaAoMontarTipo(e.to_string()))?;
    let time = time_with_offset_format
        .assume_offset(offset_for_composition_time);
    
    Ok(time)
    
}