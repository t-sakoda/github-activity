use github_activity::handler::commit_comment_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "CommitCommentEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "comment": {
                    "commit_id": "abc123"
                }
            }
        });

        assert_eq!(
            commit_comment_event::activity(&event),
            "- Commented on commit abc123 in octocat/Hello-World"
        );
    }
}
