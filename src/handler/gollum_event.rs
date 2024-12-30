use serde_json::Value;
use stringcase::pascal_case;

pub fn activity(event: &Value) -> std::string::String {
    let repo_name = event["repo"]["name"].as_str().unwrap();
    let pages = event["payload"]["pages"].as_array().unwrap();

    let activities = pages
        .iter()
        .map(|page| {
            let action = page["action"].as_str().unwrap();
            let page_name = page["page_name"].as_str().unwrap();
            format!("- {} {} in {}", pascal_case(action), page_name, repo_name)
        })
        .collect::<Vec<String>>();

    return activities.join("\n");
}
