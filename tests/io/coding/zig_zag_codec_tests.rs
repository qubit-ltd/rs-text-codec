use qubit_text_codec::{
    Leb128DecodeErrorKind,
    ZigZagCodec,
};

#[test]
fn test_new_and_strict_configuration_are_exposed() {
    let mut codec = ZigZagCodec::new();

    assert!(!codec.strict());
    assert!(!ZigZagCodec::default().strict());

    codec.set_strict(true);

    assert!(codec.strict());
    assert!(ZigZagCodec::with_strict(true).strict());
}

#[test]
fn test_mapping_round_trips_fixed_width_values() {
    assert_eq!(0_u16, ZigZagCodec::encode_i16(0));
    assert_eq!(1_u16, ZigZagCodec::encode_i16(-1));
    assert_eq!(2_u16, ZigZagCodec::encode_i16(1));
    assert_eq!(i16::MIN, ZigZagCodec::decode_u16(u16::MAX));

    assert_eq!(0_u32, ZigZagCodec::encode_i32(0));
    assert_eq!(1_u32, ZigZagCodec::encode_i32(-1));
    assert_eq!(2_u32, ZigZagCodec::encode_i32(1));
    assert_eq!(i32::MIN, ZigZagCodec::decode_u32(u32::MAX));

    assert_eq!(0_u64, ZigZagCodec::encode_i64(0));
    assert_eq!(1_u64, ZigZagCodec::encode_i64(-1));
    assert_eq!(2_u64, ZigZagCodec::encode_i64(1));
    assert_eq!(i64::MIN, ZigZagCodec::decode_u64(u64::MAX));

    assert_eq!(0_u128, ZigZagCodec::encode_i128(0));
    assert_eq!(1_u128, ZigZagCodec::encode_i128(-1));
    assert_eq!(2_u128, ZigZagCodec::encode_i128(1));
    assert_eq!(i128::MIN, ZigZagCodec::decode_u128(u128::MAX));
}

#[test]
fn test_zig_zag_round_trips_buffer_values() {
    let codec = ZigZagCodec::new();
    let mut output = [0_u8; 64];
    let mut offset = 0;

    offset += codec
        .write_i16_at(&mut output, offset, -300)
        .expect("i16 should fit");
    offset += codec
        .write_i32_at(&mut output, offset, -0x1f600)
        .expect("i32 should fit");
    offset += codec
        .write_i64_at(&mut output, offset, i64::MIN)
        .expect("i64 should fit");
    offset += codec
        .write_i128_at(&mut output, offset, i128::MIN)
        .expect("i128 should fit");

    let mut index = 0;
    let (value, consumed) = codec
        .read_i16_at(&output[..offset], index)
        .expect("valid i16")
        .expect("complete i16");
    assert_eq!(-300, value);
    index += consumed;

    let (value, consumed) = codec
        .read_i32_at(&output[..offset], index)
        .expect("valid i32")
        .expect("complete i32");
    assert_eq!(-0x1f600, value);
    index += consumed;

    let (value, consumed) = codec
        .read_i64_at(&output[..offset], index)
        .expect("valid i64")
        .expect("complete i64");
    assert_eq!(i64::MIN, value);
    index += consumed;

    let (value, consumed) = codec
        .read_i128_at(&output[..offset], index)
        .expect("valid i128")
        .expect("complete i128");
    assert_eq!(i128::MIN, value);
    index += consumed;

    assert_eq!(offset, index);
}

#[test]
fn test_zig_zag_reports_incomplete_output_and_strict_read_errors() {
    let mut output = [0_u8; 1];
    let codec = ZigZagCodec::new();

    assert_eq!(None, codec.write_i16_at(&mut output, 0, -300));
    assert_eq!(
        None,
        codec
            .read_i16_at(&[0x80], 0)
            .expect("truncated ZigZag input"),
    );

    let strict = ZigZagCodec::with_strict(true);
    let error = strict
        .read_i16_at(&[0x80, 0x00], 0)
        .expect_err("non-canonical ZigZag payload should fail");
    assert_eq!(Leb128DecodeErrorKind::NonCanonical, error.kind());
}
