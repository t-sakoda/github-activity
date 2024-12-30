use github_activity::handler::fork_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "ForkEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "forkee": {
                    "full_name": "octocat/Hello-World-2"
                }
            }
        });

        assert_eq!(
            fork_event::activity(&event),
            "- Forked octocat/Hello-World to octocat/Hello-World-2"
        );
    }
}
