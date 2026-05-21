use qubit_text_codec::{
    ByteOrder,
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf16,
    Utf16ByteCodec,
};

#[test]
fn test_utf16_byte_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);

    assert_eq!(ByteOrder::LittleEndian, codec.byte_order());
    assert_eq!(Charset::UTF_16LE, codec.charset());
    assert_eq!(Utf16::MAX_BYTES_PER_CHAR, codec.max_units_per_char());
    assert_eq!(
        Utf16::MAX_BYTES_PER_CHAR,
        CharsetCodec::<u8>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf16_byte_codec_encodes_and_decodes_bytes() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(
        4,
        codec
            .encode_one('😀', &mut output, 0)
            .expect("encode pair bytes")
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        codec.decode_one(&output, 0).expect("decode pair bytes"),
    );
}

#[test]
fn test_utf16_byte_codec_decodes_bmp_and_reports_partial_or_malformed_bytes() {
    let codec = Utf16ByteCodec::new(ByteOrder::BigEndian);

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 2,
        },
        codec.decode_one(&[0x00, 0x41], 0).expect("BMP bytes"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 2,
            available: 1,
        },
        codec.decode_one(&[0x00], 0).expect("partial unit"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 2,
        },
        codec
            .decode_one(&[0xd8, 0x3d], 0)
            .expect("partial surrogate pair"),
    );

    let error = codec
        .decode_one(&[], 1)
        .expect_err("index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .decode_one(&[0xd8, 0x3d, 0x00, 0x41], 0)
        .expect_err("high surrogate followed by BMP unit should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(2, error.index());

    let error = codec
        .decode_one(&[0xde, 0x00], 0)
        .expect_err("isolated low surrogate should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(0, error.index());
}

#[test]
fn test_utf16_byte_codec_reports_small_output_buffers() {
    let codec = Utf16ByteCodec::new(ByteOrder::LittleEndian);
    let mut output = [0_u8; Utf16::MAX_BYTES_PER_CHAR];

    assert_eq!(
        2,
        codec
            .encode_one('A', &mut output, 0)
            .expect("BMP byte encoding")
    );

    let error = codec
        .encode_one('A', &mut output[..0], 1)
        .expect_err("index outside slice should fail");
    assert_eq!(CharsetEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .encode_one('😀', &mut output[..2], 0)
        .expect_err("surrogate pair needs four bytes");
    assert_eq!(CharsetEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
}
