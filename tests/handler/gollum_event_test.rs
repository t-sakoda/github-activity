use github_activity::handler::gollum_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "GollumEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "pages": [
                    {
                        "page_name": "Home",
                        "action": "created",
                        "title": "Home"
                    }
                ]
            }
        });

        assert_eq!(
            gollum_event::activity(&event),
            "- Created Home in octocat/Hello-World"
        );
    }
}
