use time::OffsetDateTime;
use crate::enums::PointKind::PointKind;
use crate::model::dto::Reading::Reading;
use crate::model::Point::Point;

pub fn mount_point_data(data: &[Reading], instant_now: &OffsetDateTime) -> Vec<Point>{
    
    let mut points = Vec::new();
    for i in data{

        let mut point_kind = PointKind::Analysis;

        if i.at > *instant_now {
            point_kind = PointKind::Forecast;
        }

        let point = Point{
            at: i.at,
            value: i.value.value,
            kind: point_kind
        };

        points.push(point);

    }
    
    points
    
}