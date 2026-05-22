use qubit_text_codec::{
    Charset,
    CharsetConvertError,
    CharsetDecodeError,
    CharsetEncodeError,
};

#[test]
fn test_charset_convert_error_wraps_decode_and_encode_errors() {
    let decode =
        CharsetConvertError::from(CharsetDecodeError::malformed_sequence(Charset::UTF_8, 2));
    assert!(
        decode
            .to_string()
            .contains("Failed to decode source charset")
    );

    let encode = CharsetConvertError::from(CharsetEncodeError::buffer_too_small(
        Charset::UTF_8,
        4,
        4,
        0,
    ));
    assert!(
        encode
            .to_string()
            .contains("Failed to encode target charset")
    );
}
