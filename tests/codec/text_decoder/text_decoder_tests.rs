use qubit_text_codec::{
    ByteOrder,
    DecodeStatus,
    TextDecodeErrorKind,
    TextDecoder,
    Utf8Decoder,
    Utf16ByteDecoder,
    Utf16U16Decoder,
    Utf32ByteDecoder,
    Utf32U32Decoder,
};

#[test]
fn test_text_decoder_default_decode_next_covers_all_branches() {
    let decoder = Utf8Decoder;

    let mut index = 0;
    assert_eq!(
        Some('A'),
        decoder.decode_next(b"A", &mut index).expect("complete")
    );
    assert_eq!(1, index);
    assert_eq!(None, decoder.decode_next(b"A", &mut index).expect("EOF"));

    let mut incomplete_index = 0;
    let error = decoder
        .decode_next(&[0xe4], &mut incomplete_index)
        .expect_err("closed incomplete input must fail");
    assert_eq!(TextDecodeErrorKind::IncompleteSequence, error.kind());
    assert_eq!(1, error.index());

    let mut malformed_index = 1;
    let error = decoder
        .decode_next(&[b'A', 0x80], &mut malformed_index)
        .expect_err("malformed input must be offset by the cursor");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    let mut out_of_bounds = 2;
    let error = decoder
        .decode_next(b"A", &mut out_of_bounds)
        .expect_err("out-of-bounds input index must fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(2, error.index());

    assert_eq!(
        DecodeStatus::Complete {
            value: '中',
            consumed: 3,
        },
        Utf8Decoder
            .decode_prefix("A中".as_bytes(), 1)
            .expect("UTF-8 offset decode"),
    );

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 4,
            available: 1,
        },
        Utf8Decoder
            .decode_prefix(&[b'A', 0xe4], 1)
            .expect("partial UTF-8 offset needs more"),
    );

    let error = Utf8Decoder
        .decode_prefix(&[b'A', 0x80], 1)
        .expect_err("invalid UTF-8 offset should fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(1, error.index());

    assert_eq!(
        DecodeStatus::Complete {
            value: '中',
            consumed: 1,
        },
        Utf16U16Decoder
            .decode_prefix(&[0x0041, '中' as u16], 1)
            .expect("UTF-16 unit offset decode"),
    );

    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 2,
        },
        Utf16U16Decoder
            .decode_prefix(&[0x0041, 0xd83d, 0xde00], 1)
            .expect("UTF-16 unit pair offset decode"),
    );

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 3,
            available: 1,
        },
        Utf16U16Decoder
            .decode_prefix(&[0x0041, 0xd83d], 1)
            .expect("UTF-16 unit offset needs more"),
    );

    assert_eq!(
        DecodeStatus::Complete {
            value: '😀',
            consumed: 4,
        },
        Utf16ByteDecoder::new(ByteOrder::LittleEndian)
            .decode_prefix(&[0x41, 0x00, 0x3d, 0xd8, 0x00, 0xde], 2)
            .expect("UTF-16 byte offset decode"),
    );

    assert_eq!(
        DecodeStatus::NeedMore {
            required: 6,
            available: 2,
        },
        Utf16ByteDecoder::new(ByteOrder::LittleEndian)
            .decode_prefix(&[0x41, 0x00, 0x3d, 0xd8], 2)
            .expect("UTF-16 byte offset needs more"),
    );

    assert_eq!(
        DecodeStatus::Complete {
            value: '中',
            consumed: 1,
        },
        Utf32U32Decoder
            .decode_prefix(&['A' as u32, '中' as u32], 1)
            .expect("UTF-32 unit offset decode"),
    );

    assert_eq!(
        DecodeStatus::Complete {
            value: '中',
            consumed: 4,
        },
        Utf32ByteDecoder::new(ByteOrder::BigEndian)
            .decode_prefix(&[0x00, 0x00, 0x00, 0x41, 0x00, 0x00, 0x4e, 0x2d], 4)
            .expect("UTF-32 byte offset decode"),
    );

    let error = Utf32ByteDecoder::new(ByteOrder::LittleEndian)
        .decode_prefix(&[0x00, 0x00], 4)
        .expect_err("UTF-32 byte out-of-range offset should fail");
    assert_eq!(TextDecodeErrorKind::MalformedSequence, error.kind());
    assert_eq!(4, error.index());
}

#[test]
fn test_text_decoder_prefix_reports_oob_index_in_errors() {
    assert_eq!(
        TextDecodeErrorKind::MalformedSequence,
        Utf8Decoder
            .decode_prefix(b"A", 2)
            .expect_err("UTF-8 out-of-range decode should fail")
            .kind()
    );
    assert_eq!(
        2,
        Utf8Decoder
            .decode_prefix(b"A", 2)
            .expect_err("UTF-8 out-of-range decode should fail")
            .index()
    );

    assert_eq!(
        TextDecodeErrorKind::MalformedSequence,
        Utf16U16Decoder
            .decode_prefix(&[0x0041], 2)
            .expect_err("UTF-16 out-of-range decode should fail")
            .kind()
    );
    assert_eq!(
        2,
        Utf16U16Decoder
            .decode_prefix(&[0x0041], 2)
            .expect_err("UTF-16 out-of-range decode should fail")
            .index()
    );

    let utf16_bytes = [0x41, 0x00];
    assert_eq!(
        TextDecodeErrorKind::MalformedSequence,
        Utf16ByteDecoder::new(ByteOrder::LittleEndian)
            .decode_prefix(&utf16_bytes, 3)
            .expect_err("UTF-16 bytes out-of-range decode should fail")
            .kind()
    );
    assert_eq!(
        3,
        Utf16ByteDecoder::new(ByteOrder::LittleEndian)
            .decode_prefix(&utf16_bytes, 3)
            .expect_err("UTF-16 bytes out-of-range decode should fail")
            .index()
    );

    assert_eq!(
        TextDecodeErrorKind::MalformedSequence,
        Utf32U32Decoder
            .decode_prefix(&['A' as u32], 2)
            .expect_err("UTF-32 out-of-range decode should fail")
            .kind()
    );
    assert_eq!(
        2,
        Utf32U32Decoder
            .decode_prefix(&['A' as u32], 2)
            .expect_err("UTF-32 out-of-range decode should fail")
            .index()
    );

    assert_eq!(
        DecodeStatus::Complete {
            value: 'A',
            consumed: 2,
        },
        Utf16ByteDecoder::new(ByteOrder::LittleEndian)
            .decode_prefix(&[0x41, 0x00], 0)
            .expect("UTF-16 bytes decode BMP char"),
    );
}
