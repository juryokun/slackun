use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{collections::HashMap, sync::Mutex};
use structopt::StructOpt;

mod exit;
use exit::*;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(subcommand)]
    cmd: SubCommands,
    #[structopt(short = "c", long = "channel", default_value = "default")]
    channel: String,
}

#[derive(StructOpt, Debug)]
enum SubCommands {
    #[structopt(name = "post")]
    Post(Post),
    #[structopt(name = "get")]
    Get(Get),
}

#[derive(StructOpt, Debug)]
struct Post {
    message: String,
}

#[derive(StructOpt, Debug)]
struct Get {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    channels: HashMap<String, String>,
}

static CONFIG_DATA: Lazy<Mutex<Config>> = Lazy::new(|| {
    let reader = BufReader::new(File::open("rsc/setting.json").unwrap());
    let config: Config = serde_json::from_reader(reader).unwrap();
    Mutex::new(config)
});

#[tokio::main]

async fn main() {
    let args = Cli::from_args();
    match controll_subcommands(args.cmd, args.channel).await {
        Ok(exit_code) => exit_command(exit_code),
        Err(e) => {
            println!("{}", e);
            exit_command(ExitCode::ERROR)
        }
    }
}

async fn controll_subcommands(
    command: SubCommands,
    channel: String,
) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let url = lookup_url(channel).unwrap();
    match command {
        SubCommands::Post(Post { message }) => {
            let body = generate_post_body(message);
            post(url, body).await
        }
        SubCommands::Get(Get { message }) => {
            println!("{}", message);
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn lookup_url(channel: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = CONFIG_DATA.lock().unwrap();

    let url = config.channels.get(&channel).unwrap();
    Ok(url.clone())
}

fn generate_post_body(message: String) -> String {
    format!("{{\"text\": \"{}\" }}", message)
}

async fn post(url: String, body: String) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let res = client
        .post(url)
        .body(body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let result = res.text().await?;

    println!("{}", result);

    Ok(ExitCode::SUCCESS)
}

#[tokio::test]
async fn test_post_request() {
    let url = "http://httpbin.org/post".to_string();
    let body = generate_post_body("test".to_string());
    post(url, body).await;

    // {
    //   "args": {},
    //   "data": "arbitrary text",
    //   "files": {},
    //   "form": {},
    //   "headers": {
    //     "Accept": "*/*",
    //     "Content-Length": "14",
    //     "Host": "httpbin.org",
    //     "X-Amzn-Trace-Id": "Root=1-604538df-218a5bb97264e7130c298b23",
    //     "X-Person-First": "Foo!",
    //     "X-Person-Last": "Bar!!"
    //   },
    //   "json": null,
    //   "method": "POST",
    //   "origin": "49.206.4.160",
    //   "url": "https://httpbin.org/anything"
    // }
}

#[test]
fn test_global_config() {
    let global_config = CONFIG_DATA.lock().unwrap();

    assert_eq!(
        global_config.channels.get("test").unwrap(),
        "https://httpbin.org/anything"
    );
}
