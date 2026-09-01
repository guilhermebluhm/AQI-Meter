use time::OffsetDateTime;
use crate::enums::Pollutant::Pollutant;
use crate::model::dto::Concentration::Concentration;

#[derive(Debug, Copy, Clone)]
pub struct Reading{
    pub pollutant: Pollutant,
    pub value: Concentration,
    pub at: OffsetDateTime
}