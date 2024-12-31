use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let member = event["payload"]["member"]["login"].as_str().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!("- {} {} to {}", pascal_case(action), member, repo_name);
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::handler::member_event::activity;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "MemberEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "added",
                "member": {
                    "login": "octocat"
                }
            }
        });

        assert_eq!(activity(&event), "- Added octocat to octocat/Hello-World");
    }
}
