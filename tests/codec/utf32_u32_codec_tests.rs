use qubit_text_codec::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf32,
    Utf32U32Codec,
};

#[test]
fn test_utf32_u32_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf32U32Codec;

    assert_eq!(Charset::UTF_32, codec.charset());
    assert_eq!(Charset::UTF_32, CharsetCodec::<u32>::charset(&codec));
    assert_eq!(Utf32::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
    assert_eq!(
        Utf32::MAX_UNITS_PER_CHAR,
        CharsetCodec::<u32>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf32_u32_codec_encodes_and_decodes_units() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(
        1,
        codec
            .encode_one('😀', &mut output, 0)
            .expect("encode unit codec")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 1,
        },
        codec.decode_one(&output, 0).expect("decode unit codec"),
    );
}

#[test]
fn test_utf32_u32_codec_reports_partial_invalid_and_small_buffers() {
    let codec = Utf32U32Codec;
    let mut output = [0_u32; Utf32::MAX_UNITS_PER_CHAR];

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 1,
            available: 0,
        },
        codec.decode_one(&[], 0).expect("empty input needs more"),
    );

    let error = codec
        .decode_one(&[], 1)
        .expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0x110000], 0)
        .expect_err("non-scalar UTF-32 unit should fail");
    assert_eq!(CharsetDecodeErrorKind::InvalidCodePoint, error.kind());
    assert_eq!(Some(0x110000), error.value());

    let error = codec
        .encode_one('A', &mut output[..0], 0)
        .expect_err("empty output should fail");
    assert_eq!(CharsetEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(0, error.index());
}
