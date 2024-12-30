use github_activity::handler::watch_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "WatchEvent",
            "repo": {
                "name": "octocat/Hello-World"
            }
        });

        assert_eq!(
            watch_event::activity(&event),
            "- Starred octocat/Hello-World"
        );
    }
}
