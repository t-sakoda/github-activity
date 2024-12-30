use github_activity::handler::issue_comment_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "IssueCommentEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "created",
                "issue": {
                    "number": 1
                }
            }
        });

        assert_eq!(
            issue_comment_event::activity(&event),
            "- Created issue #1 in octocat/Hello-World"
        );
    }
}
