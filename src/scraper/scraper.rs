use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
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
    pub price: String,
    pub um: String,
    #[serde(skip)]
    #[allow(dead_code)]
    pub category: Category,
}

pub async fn run(url: &str) {
    let folder_name = "scraped_data";
    let file_path = format!("{}/metal_prices.json", folder_name);

    println!("Start Sync");

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

    let start = SystemTime::now();
    let timestamp = start
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut final_output = serde_json::Map::new();
    final_output.insert("time".to_string(), serde_json::json!(timestamp));

    for (cat_name, products) in categorized_data {
        final_output.insert(cat_name, serde_json::json!(products));
    }

    if let Err(e) = fs::create_dir_all(folder_name) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

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
        let um_selector = Selector::parse(".product-um")?; // Selector for Unit of Measure

        for product_el in document.select(&product_selector) {
            let product_name = if let Some(name_el) = product_el.select(&name_selector).next() {
                name_el.text().collect::<String>().trim().to_string()
            } else {
                continue;
            };

            let price = if let Some(price_el) = product_el.select(&price_selector).next() {
                let html_content = price_el.inner_html();

                let mut parsed_price = 0.0;

                if let Some(sup_start) = html_content.find("<sup>") {
                    let main_html = &html_content[..sup_start];
                    let main_digits: String = main_html.chars().filter(|c| c.is_ascii_digit()).collect();

                    let sup_content = &html_content[sup_start..];
                    if let Some(sup_end_tag) = sup_content.find("</sup>") {
                        if let Some(sup_val_start) = sup_content.find('>') {
                            let sup_digits: String = sup_content[sup_val_start + 1..sup_end_tag]
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect();

                            let combined = format!("{}.{}", main_digits, sup_digits);
                            parsed_price = combined.parse::<f32>().unwrap_or(0.0);
                        }
                    }
                }

                if parsed_price == 0.0 {
                    let raw_text = price_el.text().collect::<String>();
                    let cleaned: String = raw_text
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect();

                    if cleaned.len() > 2 {
                        let (main, dec) = cleaned.split_at(cleaned.len() - 2);
                        parsed_price = format!("{}.{}", main, dec).parse::<f32>().unwrap_or(0.0);
                    } else if !cleaned.is_empty() {
                        parsed_price = cleaned.parse::<f32>().unwrap_or(0.0) / 100.0;
                    }
                }

                // Clean up float rounding artifacts (e.g., 6.349999 -> 6.35)
                let rounded = (parsed_price * 100.0).round() / 100.0;
                format!("{:.2}", rounded)
            } else {
                "0.00".to_string()
            };

            // Extract Unit of Measure (e.g., "Pret / KG" or "Pret / ML")
            let um = if let Some(um_el) = product_el.select(&um_selector).next() {
                um_el.text().collect::<String>().trim().to_string()
            } else {
                "".to_string()
            };

            all_products.push(MetalData {
                name: product_name,
                price,
                um,
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