use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    username: String,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let url = format!("https://api.github.com/users/{}/events", args.username);
    dbg!("URL: {}", &url);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    dbg!(&body);

    let obj: serde_json::Value = serde_json::from_str(&body)?;
    dbg!(&obj);

    Ok(())
}
