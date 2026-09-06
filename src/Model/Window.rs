use time::{Duration, OffsetDateTime};

#[derive(Copy, Clone, Debug)]
pub struct Window{
    pub duration: Duration,
    pub ending_at: OffsetDateTime,
    pub min_coverege: f64
}