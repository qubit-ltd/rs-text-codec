use qubit_text_codec::CharsetEncodeErrorKind;

#[test]
fn test_charset_encode_error_kind_displays_messages() {
    assert_eq!(
        "The code point is not a valid Unicode scalar value.",
        CharsetEncodeErrorKind::InvalidCodePoint.to_string(),
    );
    assert_eq!(
        "The character cannot be represented by the target encoding.",
        CharsetEncodeErrorKind::UnmappableCharacter.to_string(),
    );
    assert_eq!(
        "The input character index is outside the input buffer.",
        CharsetEncodeErrorKind::InvalidInputIndex.to_string(),
    );
    assert_eq!(
        "The output buffer is too small.",
        CharsetEncodeErrorKind::BufferTooSmall.to_string(),
    );
}
