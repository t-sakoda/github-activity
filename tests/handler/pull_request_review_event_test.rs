use github_activity::handler::pull_request_review_comment_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PullRequestReviewCommentEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "created",
                "pull_request": {
                    "number": 1347
                },
                "comment": {
                    "body": "Nice change",
                    "commit_id": "abc123"
                }
            }
        });

        assert_eq!(
            pull_request_review_comment_event::activity(&event),
            "- Created a comment on pull request #1347 in octocat/Hello-World"
        );
    }
}
