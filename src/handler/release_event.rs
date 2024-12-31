use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let release_name = event["payload"]["release"]["name"].as_str().unwrap();
    return format!("- Released {} in {}", release_name, repo_name);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::release_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "ReleaseEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "published",
                "release": {
                    "tag_name": "v1.0.0",
                    "name": "v1.0.0"
                }
            }
        });

        assert_eq!(activity(&event), "- Released v1.0.0 in octocat/Hello-World");
    }
}
