use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let commit_id = event["payload"]["comment"]["commit_id"].as_str().unwrap();

    return format!("- Commented on commit {} in {}", commit_id, repo_name);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::commit_comment_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "CommitCommentEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "comment": {
                    "commit_id": "abc123"
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Commented on commit abc123 in octocat/Hello-World"
        );
    }
}
