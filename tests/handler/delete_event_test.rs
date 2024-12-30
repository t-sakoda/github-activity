use github_activity::handler::delete_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "DeleteEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "ref": "refs/heads/feature",
                "ref_type": "branch"
            }
        });

        assert_eq!(
            delete_event::activity(&event),
            "- Deleted branch refs/heads/feature from octocat/Hello-World"
        );
    }
}
