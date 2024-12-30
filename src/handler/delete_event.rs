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
