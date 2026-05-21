use qubit_unicode::{
    TextCodingError,
    TextDecodingError,
    TextDecodingErrorKind,
    TextEncoding,
    TextEncodingError,
    TextEncodingErrorKind,
};

#[test]
fn test_text_coding_error_wraps_encoding_and_decoding_errors() {
    const GBK: TextEncoding = TextEncoding::new("gbk", "GBK", &["cp936"]);

    let decoding = TextDecodingError::new(
        TextEncoding::UTF_32,
        TextDecodingErrorKind::InvalidCodePoint,
        0,
    );
    let encoding = TextEncodingError::new(GBK, TextEncodingErrorKind::UnmappableCharacter, 1);

    assert!(matches!(
        TextCodingError::from(decoding),
        TextCodingError::Decoding(_)
    ));
    assert!(matches!(
        TextCodingError::from(encoding),
        TextCodingError::Encoding(_)
    ));
}
