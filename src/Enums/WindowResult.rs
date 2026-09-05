use crate::model::Aggregate::Aggregate;

pub enum WindowResult {
    Complete(Aggregate),
    Partial(Aggregate),
    Insuficient{expected: usize, present: usize},
    NoData
}