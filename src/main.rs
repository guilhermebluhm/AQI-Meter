use time::{Duration, OffsetDateTime, UtcOffset};
use crate::errors::AppError::AppError;
use crate::model::Window::Window;
use crate::utils::ConverterAirQualityRawForReading::{at_hour, to_reading};
use crate::utils::HttpClient::http_client;
use crate::utils::MountSeries::aggregate;
use crate::utils::SeriesAndAgreggateFunction::to_series;

mod model;
mod utils;
mod errors;
mod enums;

fn main() -> Result<(), AppError> {

    let aqi = http_client(-25.4284, -49.2733)
        .map_err(|e| AppError::FalhaMontagemDadosAplicacao(e.to_string()))?;
    let lista_montada = to_reading(&aqi)
        .map_err(|e| AppError::FalhaMontagemDadosAplicacao(e.to_string()))?;

    let timestamp = OffsetDateTime::now_local().map_err(|e|
        AppError::FalhaMontagemDadosAplicacao(e.to_string()))?;
    let offset = UtcOffset::from_whole_seconds(aqi.utc_offset_seconds as i32).map_err(|e|
        AppError::FalhaMontagemDadosAplicacao(e.to_string()))?;
    let time_with_offset = timestamp.replace_offset(offset);

    /*
    a massa de dados retornado pelo AQI conta com as ultimas 24h + datas futuras
    quando se aplica o recorte binario baseado na data corrente (time_with_offset)
    se vê o última ponto da leitura realizada
    */
    println!("{}", time_with_offset);
    let idx_corte = lista_montada.partition_point(|p| p.at <= time_with_offset);
    if idx_corte == 0{
        return Err(AppError::FalhaMontagemDadosAplicacao("Não houve nenhuma correspondencia no corte de datas".to_string()))
    }
    let horario_corte = lista_montada.get(idx_corte-1).ok_or_else(||{
       "Falha ao capturar o corte na data".to_string()
    }).map_err(|e| AppError::FalhaMontagemDadosAplicacao(e.to_string()))?;
    let lista_filtrada = at_hour(&lista_montada, horario_corte.at);
    println!("{:#?}", lista_filtrada);
    
    let series_data = to_series(&lista_montada, horario_corte.at);
    if let Ok(x) = series_data{
        let w = Window{
            duration: Duration::hours(24),
            ending_at: horario_corte.at,
            min_coverege: 0.75
        };
        for i in x.values(){
            aggregate(i, &w);
        }
    }

    Ok(())

}
