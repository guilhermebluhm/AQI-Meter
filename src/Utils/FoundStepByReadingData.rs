use std::collections::HashMap;
use time::OffsetDateTime;

pub fn found_step_by_reading_data(data: &HashMap<String, i64>) -> i64{

    let mut contador_frequencia = HashMap::new();
    for i in data.values(){
        *contador_frequencia.entry(i+1).or_insert(0) += 1;
    }
    *contador_frequencia.iter().max_by_key(|(_,v)| *v).unwrap().0
    
}