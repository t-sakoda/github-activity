use github_activity::handler::public_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "PublicEvent",
            "repo": {
                "name": "octocat/Hello-World"
            }
        });

        assert_eq!(
            public_event::activity(&event),
            "- Made octocat/Hello-World public"
        );
    }
}
