#[cfg(test)]
mod tests {
    use crate::Event;

    #[test]
    fn callback_event_keeps_unknown_object_fields() {
        let event: Event = serde_json::from_value(serde_json::json!({
            "type": "message_new",
            "group_id": 123,
            "object": {
                "message": {
                    "id": 7,
                    "peer_id": 456,
                    "text": "Привет",
                    "new_vk_field": "value"
                }
            },
            "event_id": "event-1"
        }))
        .unwrap();

        let message = event.message().unwrap();
        assert_eq!(message.id, Some(7));
        assert_eq!(message.peer_id, Some(456));
        assert_eq!(message.text.as_deref(), Some("Привет"));
        assert_eq!(
            message.extra.get("new_vk_field").and_then(|value| value.as_str()),
            Some("value")
        );
    }

    #[test]
    fn event_without_message_returns_none() {
        let event: Event = serde_json::from_value(serde_json::json!({
            "type": "group_join",
            "group_id": 123,
            "object": { "user_id": 456 }
        }))
        .unwrap();

        assert!(event.message().is_none());
    }
}
