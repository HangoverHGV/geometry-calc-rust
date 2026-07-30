use std::path::PathBuf;

pub const BASE_URL: &str = "https://www.trutzi.ro/";
pub const START_URL: &str = "https://www.trutzi.ro/profile-laminate-tevi-si-tabla";

/// Returns an absolute, cross-platform path for the scraped JSON file
pub fn get_json_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = std::env::current_dir()?;
    path.push("scraped_data");
    std::fs::create_dir_all(&path)?; // Creates the folder automatically if missing
    path.push("metal_prices.json");
    Ok(path)
}