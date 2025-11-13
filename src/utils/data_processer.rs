use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;
use serde::Deserialize;


pub(crate) type Data = HashMap<String, CityData>;
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct CityData {
    pub city_name: String,
    pub lat: f64,
    pub lon: f64,
    pub x_km: f64,
    pub y_km: f64,
}

#[derive(Debug, Deserialize)]
pub struct CityRecord {
    pub city: String,
    pub lat: f64,
    pub lon: f64,
    pub  x_km: f64,
    pub y_km: f64,
}

pub fn load_cities_data(file_path: &str) -> Result<Data, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut cities_data = HashMap::new();

    for result in rdr.deserialize() {
        let record: CityRecord = result?;
        cities_data.insert(record.city.clone(), CityData {
            city_name: record.city,
            lat: record.lat,
            lon: record.lon,
            x_km: record.x_km,
            y_km: record.y_km,
        });
    }

    Ok(cities_data)
}
