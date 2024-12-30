use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let member = event["payload"]["member"]["login"].as_str().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!("- {} {} to {}", pascal_case(action), member, repo_name);
}
