use github_activity::handler::create_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "CreateEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "ref": "refs/heads/feature",
                "ref_type": "branch"
            }
        });

        assert_eq!(
            create_event::activity(&event),
            "- Created branch refs/heads/feature on octocat/Hello-World"
        );
    }
}
