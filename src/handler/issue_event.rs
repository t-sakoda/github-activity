use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let issue = event["payload"]["issue"]["number"].as_u64().unwrap();
    let title = event["payload"]["issue"]["title"].as_str().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!(
        "- {} issue #{}: {} in {}",
        pascal_case(action),
        issue,
        title,
        repo_name
    );
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::issue_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "IssuesEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "opened",
                "issue": {
                    "number": 1347,
                    "title": "Found a bug"
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Opened issue #1347: Found a bug in octocat/Hello-World"
        );
    }
}
