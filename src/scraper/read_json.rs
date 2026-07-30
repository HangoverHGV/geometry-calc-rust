use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use super::constants::get_json_file_path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetalData {
    pub name: String,
    pub price: String,
    pub um: String,
}

/// Reads and parses the scraped metal prices JSON file.
/// Returns a map of category names to vectors of MetalData, plus the timestamp.
pub fn load_metal_prices() -> Result<(u64, std::collections::HashMap<String, Vec<MetalData>>), Box<dyn std::error::Error>> {
    let file_path = get_json_file_path()?;

    if !file_path.exists() {
        return Err(format!("JSON file not found at: {}", file_path.display()).into());
    }

    // Open file and wrap in a BufReader for efficient memory reading of larger files (144KB+)
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Parse into a dynamic JSON Value first to extract the timestamp and dynamic categories easily
    let json_val: serde_json::Value = serde_json::from_reader(reader)?;

    let mut timestamp = 0;
    let mut categories = std::collections::HashMap::new();

    if let Some(obj) = json_val.as_object() {
        for (key, value) in obj {
            if key == "time" {
                timestamp = value.as_u64().unwrap_or(0);
            } else {
                // Deserialize the array of products for each category
                match serde_json::from_value::<Vec<MetalData>>(value.clone()) {
                    Ok(products) => {
                        categories.insert(key.clone(), products);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse category '{}': {}", key, e);
                    }
                }
            }
        }
    }

    Ok((timestamp, categories))
}