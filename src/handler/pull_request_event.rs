use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let pr = event["payload"]["number"].as_u64().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    match action {
        "review_requested" => {
            return format!("- Review requested pull request #{} in {}", pr, repo_name,);
        }
        "review_requeste_removed" => {
            return format!(
                "- Review request removed from pull request #{} in {}",
                pr, repo_name
            );
        }
        _ => {
            return format!(
                "- {} pull request #{} in {}",
                pascal_case(action),
                pr,
                repo_name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::pull_request_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PullRequestEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "opened",
                "number": 1,
                "pull_request": {
                    "title": "Update the README with new information"
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Opened pull request #1 in octocat/Hello-World"
        );
    }
}
