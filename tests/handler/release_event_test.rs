use github_activity::handler::release_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "ReleaseEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "published",
                "release": {
                    "tag_name": "v1.0.0",
                    "name": "v1.0.0"
                }
            }
        });

        assert_eq!(
            release_event::activity(&event),
            "- Released v1.0.0 in octocat/Hello-World"
        );
    }
}
