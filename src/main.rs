mod function;

use std::io::Write;
use actix_web::{post, web, App, HttpResponse, HttpServer, Result};
use env_logger;
use serde::Deserialize;
use chrono::Local;
use std::sync::LazyLock;

const NONE: Option<&str> = None;
static START_TIME: LazyLock<String> = LazyLock::new(|| Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
static LOG_FILE_NAME: LazyLock<String> = LazyLock::new(|| format!("log_{}.log", Local::now().format("%Y-%m-%d_%H-%M-%S")));


#[derive(Deserialize)]
struct SetId {
    id: i128,
}

#[derive(Deserialize)]
struct Config {
    port: u16,
    logging: bool,
}

#[post("/")]
async fn index(req: web::Json<SetId>) -> Result<HttpResponse> {
    let local = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("users.txt")?;
    file.write_all(format!("User ID: {}\n", req.id).as_bytes())?;

    function::add_log(&LOG_FILE_NAME, "Info", "Add user ID", &local, NONE)?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;

    if config.logging == true {
        env_logger::init();
    }
    
    std::fs::File::create(&LOG_FILE_NAME.to_string())?;

    function::add_log(&LOG_FILE_NAME, "Power ON", "Server started", &START_TIME, Some(true))?;
    
    HttpServer::new(|| App::new()
        .service(index))
        .bind(("127.0.0.1", config.port))?
        .run()
        .await?;

    Ok(())
}