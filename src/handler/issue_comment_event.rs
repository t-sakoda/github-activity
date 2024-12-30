use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let issue = event["payload"]["issue"]["number"].as_u64().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    return format!(
        "- {} issue #{} in {}",
        pascal_case(action),
        issue,
        repo_name
    );
}
