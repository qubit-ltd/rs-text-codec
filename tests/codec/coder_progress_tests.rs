use qubit_text_codec::{
    CoderProgress,
    CoderStatus,
};

#[test]
fn test_coder_progress_exposes_status_and_counts() {
    let complete = CoderProgress::complete(2, 3);
    assert_eq!(CoderStatus::Complete, complete.status());
    assert_eq!(2, complete.read());
    assert_eq!(3, complete.written());
    assert_eq!(0, complete.required());
    assert_eq!(None, complete.index());
    assert_eq!(0, complete.available());

    assert!(matches!(
        CoderProgress::need_input(1, 1, 0, 0, 0).status(),
        CoderStatus::NeedInput { .. },
    ));
    let need_input = CoderProgress::need_input(1, 2, 4, 3, 1);
    assert_eq!(3, need_input.required());
    assert_eq!(Some(4), need_input.index());
    assert_eq!(1, need_input.available());

    assert!(matches!(
        CoderProgress::need_output(1, 0, 0, 0, 0).status(),
        CoderStatus::NeedOutput { .. },
    ));
    let need_output = CoderProgress::need_output(5, 6, 7, 8, 9);
    assert_eq!(8, need_output.required());
    assert_eq!(Some(7), need_output.index());
    assert_eq!(9, need_output.available());
}
