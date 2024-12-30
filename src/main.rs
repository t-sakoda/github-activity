use clap::Parser;

mod handler;

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

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    let events: serde_json::Value = serde_json::from_str(&body)?;

    println!("Output:");
    for event in events.as_array().unwrap() {
        let event_type = event["type"].as_str().unwrap();
        match event_type {
            "CommitCommentEvent" => {
                println!("{}", handler::commit_comment_event::activity(&event));
            }
            "CreateEvent" => {
                println!("{}", handler::create_event::activity(&event));
            }
            "DeleteEvent" => {
                println!("{}", handler::delete_event::activity(&event));
            }
            "ForkEvent" => {
                println!("{}", handler::fork_event::activity(&event));
            }
            "GollumEvent" => {
                println!("{}", handler::gollum_event::activity(&event));
            }
            "IssueCommentEvent" => {
                println!("{}", handler::issue_comment_event::activity(&event));
            }
            "IssuesEvent" => {
                println!("{}", handler::issue_event::activity(&event));
            }
            "MemberEvent" => {
                println!("{}", handler::member_event::activity(&event));
            }
            "PublicEvent" => {
                println!("{}", handler::public_event::activity(&event));
            }
            "PullRequestEvent" => {
                println!("{}", handler::pull_request_event::activity(&event));
            }
            "PullRequestReviewEvent" => {
                println!("{}", handler::pull_request_review_event::activity(&event));
            }
            "PullRequestReviewCommentEvent" => {
                println!(
                    "{}",
                    handler::pull_request_review_comment_event::activity(&event)
                );
            }
            "PullRequestReviewThreadEvent" => {
                println!(
                    "{}",
                    handler::pull_request_review_thread_event::activity(&event)
                );
            }
            "PushEvent" => {
                println!("{}", handler::push_event::activity(&event));
            }
            "ReleaseEvent" => {
                println!("{}", handler::release_event::activity(&event));
            }
            "SponsorshipEvent" => {
                println!("- Created sponsorship");
            }
            "WatchEvent" => {
                println!("{}", handler::watch_event::activity(&event));
            }
            _ => {
                println!("{}: {}", event_type, event);
            }
        }
    }

    Ok(())
}
