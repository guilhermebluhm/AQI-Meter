use time::{Duration, OffsetDateTime};

#[derive(Copy, Clone)]
pub struct Window{
    pub duration: Duration,
    pub ending_at: OffsetDateTime,
    pub min_coverege: f64
}