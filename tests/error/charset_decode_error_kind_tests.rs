use qubit_text_codec::CharsetDecodeErrorKind;

#[test]
fn test_charset_decode_error_kind_displays_messages() {
    assert_eq!(
        "The encoded text sequence is malformed.",
        CharsetDecodeErrorKind::MalformedSequence.to_string(),
    );
    assert_eq!(
        "The encoded text sequence is incomplete.",
        CharsetDecodeErrorKind::IncompleteSequence.to_string(),
    );
    assert_eq!(
        "The decoded code point is not a valid Unicode scalar value.",
        CharsetDecodeErrorKind::InvalidCodePoint.to_string(),
    );
}
