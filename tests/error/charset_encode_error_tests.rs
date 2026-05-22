use qubit_text_codec::{
    Charset,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
};

#[test]
fn test_charset_encode_error_exposes_context() {
    const GBK: Charset = Charset::new("gbk", "GBK", &["cp936"]);

    let error = CharsetEncodeError::buffer_too_small(Charset::UTF_16, 2, 4, 1);

    assert_eq!(Charset::UTF_16, error.charset());
    assert!(matches!(
        error.kind(),
        CharsetEncodeErrorKind::BufferTooSmall { .. },
    ));
    assert_eq!(2, error.index());
    assert_eq!(None, error.value());
    assert_eq!(7, error.offset_by(5).index());
    assert_eq!(
        "UTF-16 encoding error at index 2: The output buffer is too small (required 4 units, available 1 units).",
        error.to_string(),
    );

    let invalid = CharsetEncodeError::invalid_code_point(Charset::UTF_8, 0, 0x110000);
    assert_eq!(Charset::UTF_8, invalid.charset());
    assert!(matches!(
        invalid.kind(),
        CharsetEncodeErrorKind::InvalidCodePoint { value: 0x110000 },
    ));
    assert_eq!(0, invalid.index());
    assert_eq!(Some(0x110000), invalid.value());
    assert_eq!(
        "UTF-8 encoding error at index 0 for value 0x110000: The code point is not a valid Unicode scalar value.",
        invalid.to_string(),
    );

    let unmappable = CharsetEncodeError::unmappable_character(GBK, 4, '中' as u32);
    assert_eq!(GBK, unmappable.charset());
    assert_eq!(
        CharsetEncodeErrorKind::UnmappableCharacter {
            value: '中' as u32
        },
        unmappable.kind()
    );
    assert_eq!(4, unmappable.index());
    assert_eq!(Some('中' as u32), unmappable.value());

    let invalid_index = CharsetEncodeError::invalid_input_index(Charset::UTF_8, 8);
    assert_eq!(Charset::UTF_8, invalid_index.charset());
    assert_eq!(
        CharsetEncodeErrorKind::InvalidInputIndex { input_len: 0 },
        invalid_index.kind()
    );
    assert_eq!(Some(0), invalid_index.kind().input_len());
    assert_eq!(8, invalid_index.index());
    assert_eq!(None, invalid_index.value());
}
