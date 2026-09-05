use time::OffsetDateTime;
use crate::enums::PointKind::PointKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point{
    pub at: OffsetDateTime,
    pub value: f64,
    pub kind: PointKind
}

