use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CityData {
    pub lat: f64,
    pub lon: f64,
    pub x_km: f64,
    pub y_km: f64,
}

#[derive(Debug, Deserialize)]
struct CityRecord {
    city: String,
    lat: f64,
    lon: f64,
    x_km: f64,
    y_km: f64,
}

pub fn load_cities_data(file_path: &str) -> Result<HashMap<String, CityData>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut cities_data = HashMap::new();

    for result in rdr.deserialize() {
        let record: CityRecord = result?;
        cities_data.insert(record.city, CityData {
            lat: record.lat,
            lon: record.lon,
            x_km: record.x_km,
            y_km: record.y_km,
        });
    }

    Ok(cities_data)
}
