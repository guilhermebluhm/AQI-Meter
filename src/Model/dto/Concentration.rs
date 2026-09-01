use crate::enums::Unit::Unit;
use crate::errors::GeneralError::GeneralError;

#[derive(Debug, Copy, Clone)]
pub struct Concentration{
    pub value: f64,
    pub unit: Unit
}

impl Concentration{
    pub fn new(value: f64, unit: Unit) -> Result<Concentration, GeneralError>{

        if value.is_nan() || value.is_sign_negative() || value.is_infinite() {
            return Err(GeneralError::FalhaAoMontarTipo("invalid values for concentration".to_string()));
        }

        Ok(Self{
            value,
            unit
        })
    }
}