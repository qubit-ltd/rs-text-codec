use qubit_text_codec::{
    Charset,
    CharsetCodec,
    CharsetDecodeErrorKind,
    CharsetEncodeErrorKind,
    DecodeStatus,
    Utf8,
    Utf8Codec,
};

#[test]
fn test_utf8_codec_exposes_encoder_and_decoder_contracts() {
    let codec = Utf8Codec;

    assert_eq!(Charset::UTF_8, codec.charset());
    assert_eq!(Utf8::MAX_UNITS_PER_CHAR, codec.max_units_per_char());
    assert_eq!(
        Utf8::MAX_UNITS_PER_CHAR,
        CharsetCodec::<u8>::max_units_per_char(&codec)
    );
}

#[test]
fn test_utf8_codec_encodes_and_decodes() {
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(2, codec.encode_one('é', &mut output, 0).expect("Latin-1"));
    assert!(matches!(
        codec.decode_one(&output[..2], 0).expect("decode Latin-1"),
        DecodeStatus::Complete {
            value: 'é',
            consumed: 2,
        },
    ));
    assert!(matches!(
        codec.decode_one(&[], 0).expect("empty prefix needs more"),
        DecodeStatus::NeedMore { .. },
    ));
}

#[test]
fn test_utf8_codec_decodes_all_lengths_and_partial_prefixes() {
    let codec = Utf8Codec;

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 1,
        },
        codec.decode_one(b"A", 0).expect("ASCII"),
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '中',
            consumed: 3,
        },
        codec.decode_one("中".as_bytes(), 0).expect("three bytes"),
    );
    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        codec.decode_one("😀".as_bytes(), 0).expect("four bytes"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 3,
            available: 1,
        },
        codec
            .decode_one(&[0xe4], 0)
            .expect("partial three-byte prefix"),
    );
    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 2,
        },
        codec
            .decode_one(&[0xf0, 0x90], 0)
            .expect("partial four-byte prefix"),
    );
}

#[test]
fn test_utf8_codec_reports_malformed_sequences() {
    let codec = Utf8Codec;

    let cases = [
        (&[0x80][..], 0),
        (&[0xc2, b' '][..], 1),
        (&[0xe0, 0x80, 0x80][..], 1),
        (&[0xed, 0xa0, 0x80][..], 1),
        (&[0xe1, 0x80, b' '][..], 2),
        (&[0xf0, 0x80, 0x80, 0x80][..], 1),
        (&[0xf1, 0x80, b' ', 0x80][..], 2),
        (&[0xf4, 0xc0, 0x80, 0x80][..], 1),
        (&[0xf4, 0x80, 0x80, b' '][..], 3),
        (&[0xe4, b' '][..], 1),
        (&[0xe4, 0xb8, b' '][..], 2),
        (&[0xf0, 0x90, b' '][..], 2),
    ];

    for (input, index) in cases {
        let error = codec
            .decode_one(input, 0)
            .expect_err("malformed UTF-8 should fail");
        assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
        assert_eq!(index, error.index());
    }

    let error = codec
        .decode_one(b"", 1)
        .expect_err("input index outside slice should fail");
    assert_eq!(CharsetDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());
}

#[test]
fn test_utf8_codec_encodes_all_lengths_and_reports_small_buffers() {
    let codec = Utf8Codec;
    let mut output = [0_u8; Utf8::MAX_BYTES_PER_CHAR];

    assert_eq!(1, codec.encode_one('A', &mut output, 0).expect("ASCII"));
    assert_eq!(2, codec.encode_one('é', &mut output, 0).expect("two bytes"));
    assert_eq!(
        3,
        codec.encode_one('中', &mut output, 0).expect("three bytes")
    );
    assert_eq!(
        4,
        codec.encode_one('😀', &mut output, 0).expect("four bytes")
    );

    let error = codec
        .encode_one('A', &mut output[..0], 1)
        .expect_err("output index outside slice should fail");
    assert_eq!(CharsetEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(1, error.index());

    let error = codec
        .encode_one('中', &mut output[..2], 0)
        .expect_err("short output should fail");
    assert_eq!(CharsetEncodeErrorKind::BufferTooSmall, error.kind());
    assert_eq!(2, error.index());
}
