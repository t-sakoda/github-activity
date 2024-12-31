use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let issue = event["payload"]["issue"]["number"].as_u64().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!(
        "- {} issue #{} in {}",
        pascal_case(action),
        issue,
        repo_name
    );
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::issue_comment_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "IssueCommentEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "created",
                "issue": {
                    "number": 1
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Created issue #1 in octocat/Hello-World"
        );
    }
}
