use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    return format!("- Starred {}", repo_name);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::watch_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "WatchEvent",
            "repo": {
                "name": "octocat/Hello-World"
            }
        });

        assert_eq!(activity(&event), "- Starred octocat/Hello-World");
    }
}
