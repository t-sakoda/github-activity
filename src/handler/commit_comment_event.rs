use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let commit_id = event["payload"]["comment"]["commit_id"].as_str().unwrap();

    return format!("- Commented on commit {} in {}", commit_id, repo_name);
}
