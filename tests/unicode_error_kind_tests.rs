use qubit_unicode::UnicodeErrorKind;

#[test]
fn test_unicode_error_kind_exposes_java_compatible_codes() {
    assert_eq!(-2, UnicodeErrorKind::BufferOverflow.code());
    assert_eq!(-4, UnicodeErrorKind::MalformedUnicode.code());
    assert_eq!(-5, UnicodeErrorKind::IncompleteUnicode.code());
}

#[test]
fn test_unicode_error_kind_exposes_messages() {
    assert_eq!(
        "The buffer overflows.",
        UnicodeErrorKind::BufferOverflow.message()
    );
    assert_eq!(
        "The Unicode code unit sequence is malformed.",
        UnicodeErrorKind::MalformedUnicode.message()
    );
    assert_eq!(
        "The Unicode code unit sequence is incomplete.",
        UnicodeErrorKind::IncompleteUnicode.message()
    );
}
