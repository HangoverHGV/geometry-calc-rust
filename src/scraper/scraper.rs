use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, Local};
use reqwest;
use scraper::{Html, Selector};
use super::constants::{ BASE_URL };
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Category {
    #[default]
    SquareTube,
    RectangleTube,
    RoundTube,
    MetalPlate,
    Profiles,
}

impl Category {
    pub fn to_string(&self) -> String {
        match *self {
            Category::SquareTube => "SquareTube".to_string(),
            Category::RectangleTube => "RectangleTube".to_string(),
            Category::RoundTube => "RoundTube".to_string(),
            Category::MetalPlate => "MetalPlate".to_string(),
            Category::Profiles => "Profiles".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryLinks {
    pub name: Category,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetalData {
    pub name: String,
    pub price: f32,
    #[serde(skip)]
    pub category: Category,
}

pub async fn run(url: &str) {
    let folder_name = "scraped_data";
    let file_path = format!("{}/metal_prices.json", folder_name);

    // 1. Check if file already exists and was created today
    if Path::new(&file_path).exists() {
        if let Ok(file_content) = fs::read_to_string(&file_path) {
            // Parse just the time field or whole JSON to check the timestamp
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&file_content) {
                if let Some(timestamp_sec) = json_val.get("time").and_then(|v| v.as_u64()) {
                    let file_datetime = DateTime::<Utc>::from_timestamp(timestamp_sec as i64, 0)
                        .map(|dt| dt.with_timezone(&Local))
                        .unwrap_or_else(|| Local::now());

                    let today = Local::now().date_naive();
                    let file_date = file_datetime.date_naive();

                    if file_date == today {
                        println!("Skipping execution: Scraper already ran today ({})!", today);
                        return;
                    }
                }
            }
        }
    }

    println!("Starting scraper execution for today...");

    let links = match get_links(url).await {
        Ok(links) => links,
        Err(e) => {
            eprintln!("Failed to fetch root page: {}", e);
            return;
        }
    };

    if links.is_empty() {
        eprintln!("Error: get_links returned 0 links!");
        return;
    }

    let mut categorized_data: HashMap<String, Vec<MetalData>> = HashMap::new();

    for cat_link in links {
        println!("Crawling category: {:?} -> {}", cat_link.name, cat_link.link);

        match get_all_category_products(&cat_link.link, cat_link.name).await {
            Ok(products) => {
                println!(" -> Fetched {} items.", products.len());
                let key = cat_link.name.to_string();
                categorized_data.insert(key, products);
            }
            Err(e) => {
                eprintln!(" -> Error crawling {:?}: {}", cat_link.name, e);
            }
        }
    }

    // Get current Unix timestamp
    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Build final JSON structure
    let mut final_output = serde_json::Map::new();
    final_output.insert("time".to_string(), serde_json::json!(timestamp));

    for (cat_name, products) in categorized_data {
        final_output.insert(cat_name, serde_json::json!(products));
    }

    // Create folder if it doesn't exist
    if let Err(e) = fs::create_dir_all(folder_name) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    // Save to JSON file
    let json_string = serde_json::to_string_pretty(&final_output).unwrap();

    if let Err(e) = fs::write(&file_path, json_string) {
        eprintln!("Failed to write JSON file: {}", e);
    } else {
        println!("Successfully saved scraped data to {}", file_path);
    }
}

pub async fn get_links(url: &str) -> Result<Vec<CategoryLinks>, Box<dyn std::error::Error>> {
    let mut links: Vec<CategoryLinks> = Vec::new();

    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let children_selector = Selector::parse(".children a")?;

    for element in document.select(&children_selector) {
        if let Some(href) = element.value().attr("href") {
            let full_link = format!("{}{}", BASE_URL, href);
            let text_content = href.to_lowercase();

            let category = if text_content.contains("teava-patrata") {
                Category::SquareTube
            } else if text_content.contains("teava-dreptunghiulara") {
                Category::RectangleTube
            } else if text_content.contains("teava-rotunda") {
                Category::RoundTube
            } else if text_content.ends_with("/tabla") || text_content.contains("/tabla?") {
                Category::MetalPlate
            } else if text_content.ends_with("/profile") || text_content.contains("/profile?") {
                Category::Profiles
            } else {
                continue;
            };

            links.push(CategoryLinks {
                name: category,
                link: full_link,
            });
        }
    }
    Ok(links)
}

pub async fn get_all_category_products(
    base_category_url: &str,
    category: Category,
) -> Result<Vec<MetalData>, Box<dyn std::error::Error>> {
    let mut all_products: Vec<MetalData> = Vec::new();
    let mut current_url = base_category_url.to_string();

    loop {
        let html = reqwest::get(&current_url).await?.text().await?;
        let document = Html::parse_document(&html);

        let product_selector = Selector::parse("div.product")?;
        let name_selector = Selector::parse("h2.product-name a")?;
        let price_selector = Selector::parse(".price.t-call-to-action-background-to-text")?;

        for product_el in document.select(&product_selector) {
            let product_name = if let Some(name_el) = product_el.select(&name_selector).next() {
                name_el.text().collect::<String>().trim().to_string()
            } else {
                continue;
            };

            let price = if let Some(price_el) = product_el.select(&price_selector).next() {
                let raw_text = price_el.text().collect::<String>();
                let cleaned: String = raw_text
                    .replace(',', ".")
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.')
                    .collect();
                cleaned.parse::<f32>().unwrap_or(0.0)
            } else {
                0.0
            };

            all_products.push(MetalData {
                name: product_name,
                price,
                category,
            });
        }

        let next_page_selector = Selector::parse("a.page-next")?;
        if let Some(next_el) = document.select(&next_page_selector).next() {
            if let Some(href) = next_el.value().attr("href") {
                current_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", BASE_URL, href)
                };
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(all_products)
}