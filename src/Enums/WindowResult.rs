use crate::model::Aggregate::Aggregate;

#[derive(Debug)]
pub enum WindowResult {
    Complete(Aggregate),
    Partial(Aggregate),
    Insuficient{expected: usize, present: usize},
    NoData
}