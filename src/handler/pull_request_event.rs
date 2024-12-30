use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let pr = event["payload"]["number"].as_u64().unwrap();
    let action = event["payload"]["action"].as_str().unwrap();
    match action {
        "review_requested" => {
            return format!("- Review requested pull request #{} in {}", pr, repo_name,);
        }
        "review_requeste_removed" => {
            return format!(
                "- Review request removed from pull request #{} in {}",
                pr, repo_name
            );
        }
        _ => {
            return format!(
                "- {} pull request #{} in {}",
                pascal_case(action),
                pr,
                repo_name
            );
        }
    }
}
