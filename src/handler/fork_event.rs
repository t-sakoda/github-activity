use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let forkee_name = event["payload"]["forkee"]["full_name"].as_str().unwrap();
    return format!("- Forked {} to {}", repo_name, forkee_name);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::fork_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "ForkEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "forkee": {
                    "full_name": "octocat/Hello-World-2"
                }
            }
        });

        assert_eq!(
            activity(&event),
            "- Forked octocat/Hello-World to octocat/Hello-World-2"
        );
    }
}
