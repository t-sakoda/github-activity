use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let ref_type = event["payload"]["ref_type"].as_str().unwrap();
    let ref_name = event["payload"]["ref"].as_str().unwrap();
    match ref_type {
        "branch" => {
            return format!("- Deleted branch {} from {}", ref_name, repo_name);
        }
        "tag" => {
            return format!("- Deleted tag {} from {}", ref_name, repo_name);
        }
        _ => {
            return format!("- Deleted {} from {}", ref_name, repo_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::delete_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "DeleteEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "ref": "refs/heads/feature",
                "ref_type": "branch"
            }
        });

        assert_eq!(
            activity(&event),
            "- Deleted branch refs/heads/feature from octocat/Hello-World"
        );
    }
}
