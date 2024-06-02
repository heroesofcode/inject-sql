use std::io;
use tokio;
use reqwest::{Client};

#[tokio::main]
async fn main() {
    let mut url = String::new();
    let mut payload_type = String::new();

    println!("Digite a URL");
    io::stdin().read_line(&mut url).expect("Não foi possivel ler a URL");
    let url = url.trim().to_string();

    println!("Digite o payload type");
    io::stdin().read_line(&mut payload_type).expect("Não foi possível ler a o payload type");
    let payload_type = payload_type.trim().to_string();

    if !url.is_empty() || !payload_type.is_empty() {
        if let Err(error) = validation_exist_sql_injection(&url, &payload_type).await {
            println!("Erro ao testar SQL injection: {}", error);
        }
    } else {
        println!("Os campos não pode ser vazio");
    }
}

async fn validation_exist_sql_injection(url: &str, payload: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let test_url = format!("{}{}", url, payload);
    println!("... Testando a url: {}", test_url);

    let response = client.get(test_url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;

        if body.contains("Warning: mysql_fetch_array()") {
            println!("Nessa URL é possivel fazer ataque SQL Injection");
        } else {
            println!("Nenhuma vunerabilidade encontrada");
        }
    } else {
        println!("Falha ao fazer a requisição")
    }

    Ok(())
}