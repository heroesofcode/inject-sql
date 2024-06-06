use std::io;
use std::process::exit;
use tokio;
use clap::Command;

mod response;
use response::validation_exist_sql_injection;

#[tokio::main]
async fn main() {
    check_version();

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
        if let Err(e) = validation_exist_sql_injection(&url, &payload_type).await {
            eprintln!("Error: {}", e);
        }
    } else {
        println!("Fields cannot be empty");
        exit(1);
    }
}

fn check_version() {
    let _app = Command::new("injeqtor")
        .version("0.1.0")
        .ignore_errors(true)
        .get_matches();
}