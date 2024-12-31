use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let pr = event["payload"]["pull_request"]["number"].as_u64().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!(
        "- {} pull request #{} in {}",
        pascal_case(action),
        pr,
        repo_name
    );
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::pull_request_review_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PullRequestReviewEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "created",
                "pull_request": {
                    "number": 1347
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Created pull request #1347 in octocat/Hello-World"
        );
    }
}
