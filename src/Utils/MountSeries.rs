use std::collections::HashMap;
use time::Duration;
use crate::enums::PointKind::PointKind;
use crate::enums::Pollutant::Pollutant;
use crate::enums::Unit::Unit;
use crate::enums::WindowResult::WindowResult;
use crate::model::Aggregate::Aggregate;
use crate::model::dto::Concentration::Concentration;
use crate::model::Point::Point;
use crate::model::Series::Series;
use crate::model::Window::Window;
use crate::utils::FoundStepByReadingData::found_step_by_reading_data;

pub fn mount_series(mapa_serie: &mut HashMap<Pollutant, Series>, pontos: Vec<Point>, step: i64, tipo_poluente: Pollutant) {

    let mut ser = Series{
        polluent: Pollutant::Pm25,
        unit: Unit::MicrogramsPerCubicMeter,
        points: pontos,
        step: Duration::hours(step)
    };

    if tipo_poluente == Pollutant::Pm10 {
        ser.polluent = Pollutant::Pm10;
        mapa_serie.insert(Pollutant::Pm10, ser);
    }
    else{
        mapa_serie.insert(Pollutant::Pm25, ser);
    }

}

pub fn aggregate(series: &Series, w: &Window) -> WindowResult {

    //irei ainda fazer a analise e correção da lógica
    let corte_final = series.points.partition_point(|p| p.at <= w.ending_at);
    let corte_inicial = series.points.partition_point(|p| p.at <= w.ending_at - w.duration);
    let intervalo_dados_leitura = &series.points[corte_inicial..corte_final].iter().filter(|f|
                                            f.kind == PointKind::Analysis).collect::<Vec<&Point>>();

    let n_presente = intervalo_dados_leitura.len();
    let n_esperado = (w.duration.whole_hours() / ((series.step.whole_seconds()/60)/60)) as usize;

    if n_presente.eq(&0) {
        return WindowResult::NoData
    }
    if ((n_presente / n_esperado) as f64) < w.min_coverege {
        return WindowResult::Insuficient {expected: n_esperado, present: n_presente}
    }

    let media_valores_coletados = intervalo_dados_leitura.iter().map(|m| m.value).sum::<f64>();
    let media_coletada = media_valores_coletados / n_presente as f64;

    let concentration = Concentration{
        value: media_coletada,
        unit: Unit::MicrogramsPerCubicMeter
    };

    let agg = Aggregate{
        value: concentration,
        window: *w,
        expected: n_esperado,
        present: n_presente,
        covegere: w.min_coverege,
        has_forecast: false
    };

    if n_esperado != n_presente {
        return WindowResult::Partial(agg);
    }

    WindowResult::Complete(agg)
}
