use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let forkee_name = event["payload"]["forkee"]["full_name"].as_str().unwrap();
    return format!("- Forked {} to {}", repo_name, forkee_name);
}
