use time::Duration;
use crate::enums::Pollutant::Pollutant;
use crate::enums::Unit::Unit;
use crate::model::Point::Point;

#[derive(PartialEq, Debug, Clone)]
pub struct Series{
    pub polluent: Pollutant,
    pub unit: Unit,
    pub points: Vec<Point>,
    pub step: Duration
}