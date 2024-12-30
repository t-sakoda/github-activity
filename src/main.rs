use clap::Parser;
use stringcase::pascal_case;

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
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let ref_type = event["payload"]["ref_type"].as_str().unwrap();
                let ref_name = if ref_type == "repository" {
                    ""
                } else {
                    event["payload"]["ref"].as_str().unwrap()
                };
                match ref_type {
                    "repository" => {
                        println!("- Created {}", repo_name);
                    }
                    "branch" => {
                        println!("- Created branch {} on {}", ref_name, repo_name);
                    }
                    "tag" => {
                        println!("- Created tag {} on {}", ref_name, repo_name);
                    }
                    _ => {
                        println!("- Created {} on {}", ref_name, repo_name);
                    }
                }
            }
            "DeleteEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let ref_type = event["payload"]["ref_type"].as_str().unwrap();
                let ref_name = event["payload"]["ref"].as_str().unwrap();
                match ref_type {
                    "branch" => {
                        println!("- Deleted branch {} from {}", ref_name, repo_name);
                    }
                    "tag" => {
                        println!("- Deleted tag {} from {}", ref_name, repo_name);
                    }
                    _ => {
                        println!("- Deleted {} from {}", ref_name, repo_name);
                    }
                }
            }
            "ForkEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                println!("- Forked {}", repo_name);
            }
            "GollumEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let pages = event["payload"]["pages"].as_array().unwrap();
                for page in pages {
                    let action = page["action"].as_str().unwrap();
                    let page_name = page["page_name"].as_str().unwrap();
                    println!("- {} {} in {}", pascal_case(action), page_name, repo_name);
                }
            }
            "IssueCommentEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let issue = event["payload"]["issue"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!(
                    "- {} issue #{} in {}",
                    pascal_case(action),
                    issue,
                    repo_name
                );
            }
            "IssuesEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let issue = event["payload"]["issue"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!(
                    "- {} issue #{} in {}",
                    pascal_case(action),
                    issue,
                    repo_name
                );
            }
            "MemberEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let member = event["payload"]["member"]["login"].as_str().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!("- {} {} to {}", pascal_case(action), member, repo_name);
            }
            "PublicEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                println!("- Made {} public", repo_name);
            }
            "PullRequestEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let pr = event["payload"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                match action {
                    "review_requested" => {
                        println!("- Review requested pull request #{} in {}", pr, repo_name,);
                    }
                    "review_requeste_removed" => {
                        println!(
                            "- Review request removed from pull request #{} in {}",
                            pr, repo_name
                        );
                    }
                    _ => {
                        println!(
                            "- {} pull request #{} in {}",
                            pascal_case(action),
                            pr,
                            repo_name
                        );
                    }
                }
            }
            "PullRequestReviewEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let pr = event["payload"]["pull_request"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!(
                    "- {} pull request #{} in {}",
                    pascal_case(action),
                    pr,
                    repo_name
                );
            }
            "PullRequestReviewCommentEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let pr = event["payload"]["pull_request"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!(
                    "- {} a comment on pull request #{} in {}",
                    pascal_case(action),
                    pr,
                    repo_name
                );
            }
            "PullRequestReviewThreadEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let pr = event["payload"]["pull_request"]["number"].as_u64().unwrap();
                let action = event["payload"]["action"].as_str().unwrap();
                println!(
                    "- {} a comment thread on pull request #{} in {}",
                    pascal_case(action),
                    pr,
                    repo_name
                );
            }
            "PushEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let commit_num = event["payload"]["commits"].as_array().unwrap().len();
                let commit_plural = if commit_num > 1 { "s" } else { "" };
                println!(
                    "- Pushed {} commit{} to {}",
                    commit_num, commit_plural, repo_name
                );
            }
            "ReleaseEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                let release_name = event["payload"]["release"]["name"].as_str().unwrap();
                println!("- Released {} in {}", release_name, repo_name);
            }
            "SponsorshipEvent" => {
                println!("- Created sponsorship");
            }
            "WatchEvent" => {
                let repo_name = event["repo"]["name"].as_str().unwrap();
                println!("- Starred {}", repo_name);
            }
            _ => {
                println!("{}: {}", event_type, event);
            }
        }
    }

    Ok(())
}
