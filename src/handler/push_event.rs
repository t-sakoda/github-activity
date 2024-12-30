use serde_json::Value;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let commit_num = event["payload"]["commits"].as_array().unwrap().len();
    let commit_plural = if commit_num > 1 { "s" } else { "" };
    return format!(
        "- Pushed {} commit{} to {}",
        commit_num, commit_plural, repo_name
    );
}
