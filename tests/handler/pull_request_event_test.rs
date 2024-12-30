use github_activity::handler::pull_request_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PullRequestEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "opened",
                "number": 1,
                "pull_request": {
                    "title": "Update the README with new information"
                }
            }
        });

        assert_eq!(
            pull_request_event::activity(&event),
            "- Opened pull request #1 in octocat/Hello-World"
        );
    }
}
