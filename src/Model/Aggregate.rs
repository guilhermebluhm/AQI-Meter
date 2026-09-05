use time::OffsetDateTime;
use crate::model::dto::Concentration::Concentration;
use crate::model::Window::Window;

pub struct Aggregate {
    pub value: Concentration,
    pub window: Window,
    pub expected: usize,
    pub present: usize,
    pub covegere: f64,
    pub peak_at: Option<OffsetDateTime>,
    pub has_forecast: bool,
}