use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let ref_type = event["payload"]["ref_type"].as_str().unwrap();
    let ref_name = if ref_type == "repository" {
        ""
    } else {
        event["payload"]["ref"].as_str().unwrap()
    };
    match ref_type {
        "repository" => {
            return format!("- Created {}", repo_name);
        }
        "branch" => {
            return format!("- Created branch {} on {}", ref_name, repo_name);
        }
        "tag" => {
            return format!("- Created tag {} on {}", ref_name, repo_name);
        }
        _ => {
            return format!("- Created {} on {}", ref_name, repo_name);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::create_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "CreateEvent",
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
            "- Created branch refs/heads/feature on octocat/Hello-World"
        );
    }
}
