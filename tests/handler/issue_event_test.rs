use github_activity::handler::issue_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "IssuesEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "opened",
                "issue": {
                    "number": 1347,
                    "title": "Found a bug"
                }
            }
        });

        assert_eq!(
            issue_event::activity(&event),
            "- Opened issue #1347: Found a bug in octocat/Hello-World"
        );
    }
}
