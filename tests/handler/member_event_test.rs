use github_activity::handler::member_event;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let event = json!({
            "type": "MemberEvent",
            "repo": {
                "name": "octocat/Hello-World"
            },
            "payload": {
                "action": "added",
                "member": {
                    "login": "octocat"
                }
            }
        });

        assert_eq!(
            member_event::activity(&event),
            "- Added octocat to octocat/Hello-World"
        );
    }
}
