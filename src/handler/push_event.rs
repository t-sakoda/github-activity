use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let commit_num = event["payload"]["commits"].as_array().unwrap().len();
    let commit_plural = if commit_num > 1 { "s" } else { "" };
    return format!(
        "- Pushed {} commit{} to {}",
        commit_num, commit_plural, repo_name
    );
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::push_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PushEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "commits": [
                    {
                        "message": "Update the README",
                        "sha": "abc123"
                    }
                ]
            }
        });

        assert_eq!(activity(&event), "- Pushed 1 commit to octocat/Hello-World");
    }
}
