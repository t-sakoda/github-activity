use github_activity::handler::push_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PushEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "commits": [
                    {
                        "message": "Update the README",
                        "sha": "abc123"
                    }
                ]
            }
        });

        assert_eq!(
            push_event::activity(&event),
            "- Pushed 1 commit to octocat/Hello-World"
        );
    }
}
