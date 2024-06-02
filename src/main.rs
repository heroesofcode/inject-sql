use std::io;
use tokio;

mod response;
use response::validation_exist_sql_injection;

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
    println!("1 - classical 1");
    println!("2 - classical 2");
    println!("3 - time-based");
    println!("4 - blind 1");
    println!("5 - blind 2");
    println!("6 - boolean 1");
    println!("7 - boolean 2");
    println!("8 - Get Database\n");

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