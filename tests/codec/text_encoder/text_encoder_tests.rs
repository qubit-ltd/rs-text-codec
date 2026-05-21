use qubit_text_codec::{
    ByteOrder,
    TextEncodeErrorKind,
    TextEncoder,
    Utf8,
    Utf8Encoder,
    Utf16,
    Utf16ByteEncoder,
    Utf16U16Encoder,
    Utf32,
    Utf32ByteEncoder,
    Utf32U32Encoder,
};

#[test]
fn test_text_encoder_default_encode_code_point_rejects_invalid_code_point() {
    let error = Utf8Encoder
        .encode_code_point(0x110000, &mut [0_u8; 4], 0)
        .expect_err("invalid code point must fail");

    assert_eq!(TextEncodeErrorKind::InvalidCodePoint, error.kind());
    assert_eq!(Some(0x110000), error.value());
    assert_eq!(0, error.index());
}

#[test]
fn test_text_encoder_supports_non_zero_indices_and_reports_them_in_errors() {
    let mut utf8_output = [0_u8; Utf8::MAX_BYTES_PER_CHAR + 2];
    let utf8_written = Utf8Encoder
        .encode_char('中', &mut utf8_output, 1)
        .expect("UTF-8 can be encoded at non-zero index");
    assert_eq!(3, utf8_written);
    assert_eq!("中".as_bytes(), &utf8_output[1..1 + utf8_written]);

    let mut utf16_output = [0_u16; Utf16::MAX_UNITS_PER_CHAR + 1];
    let utf16_written = Utf16U16Encoder
        .encode_char('😀', &mut utf16_output, 1)
        .expect("UTF-16 units can be encoded at non-zero index");
    assert_eq!(2, utf16_written);
    assert_eq!(0xd83d, utf16_output[1]);
    assert_eq!(0xde00, utf16_output[2]);

    let mut utf16_bytes_output = [0_u8; Utf16::MAX_BYTES_PER_CHAR + 2];
    let utf16_bytes_written = Utf16ByteEncoder::new(ByteOrder::LittleEndian)
        .encode_char('😀', &mut utf16_bytes_output, 1)
        .expect("UTF-16 bytes can be encoded at non-zero index");
    assert_eq!(4, utf16_bytes_written);
    assert_eq!([0x3d, 0xd8, 0x00, 0xde], utf16_bytes_output[1..5]);

    let mut utf32_output = [0_u32; Utf32::MAX_UNITS_PER_CHAR + 1];
    let utf32_written = Utf32U32Encoder
        .encode_char('中', &mut utf32_output, 1)
        .expect("UTF-32 units can be encoded at non-zero index");
    assert_eq!(1, utf32_written);
    assert_eq!('中' as u32, utf32_output[1]);

    let mut utf32_bytes_output = [0_u8; Utf32::MAX_BYTES_PER_CHAR + 1];
    let utf32_bytes_written = Utf32ByteEncoder::new(ByteOrder::BigEndian)
        .encode_char('中', &mut utf32_bytes_output, 1)
        .expect("UTF-32 bytes can be encoded at non-zero index");
    assert_eq!(4, utf32_bytes_written);
    assert_eq!([0x00, 0x00, 0x4e, 0x2d], utf32_bytes_output[1..5]);

    let error = Utf8Encoder
        .encode_char('中', &mut [0_u8; 2], 0)
        .expect_err("small UTF-8 buffer should fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());

    let error = Utf8Encoder
        .encode_code_point(0x110000, &mut [0_u8; 4], 1)
        .expect_err("invalid code point should preserve index in error");
    assert_eq!(TextEncodeErrorKind::InvalidCodePoint, error.kind());
    assert_eq!(1, error.index());

    let error = Utf16U16Encoder
        .encode_char('😀', &mut [0_u16; 1], 0)
        .expect_err("small UTF-16 output buffer should fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(1, error.index());

    let error = Utf32ByteEncoder::new(ByteOrder::BigEndian)
        .encode_char('A', &mut [0_u8; 3], 1)
        .expect_err("small UTF-32 bytes buffer should fail");
    assert_eq!(TextEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(3, error.index());
}
