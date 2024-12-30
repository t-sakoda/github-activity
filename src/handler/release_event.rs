use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let release_name = event["payload"]["release"]["name"].as_str().unwrap();
    return format!("- Released {} in {}", release_name, repo_name);
}
