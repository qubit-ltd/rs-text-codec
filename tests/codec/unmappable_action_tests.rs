use qubit_text_codec::UnmappableAction;

#[test]
fn test_unmappable_action_default_replaces() {
    assert_eq!(UnmappableAction::Replace, UnmappableAction::default());
}
