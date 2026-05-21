use qubit_text_codec::MalformedAction;

#[test]
fn test_malformed_action_default_replaces() {
    assert_eq!(MalformedAction::Replace, MalformedAction::default());
}
