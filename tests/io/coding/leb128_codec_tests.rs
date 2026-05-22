use qubit_text_codec::{
    Leb128Codec,
    Leb128DecodeError,
    Leb128DecodeErrorKind,
};

#[test]
fn test_new_and_strict_configuration_are_exposed() {
    let mut codec = Leb128Codec::new();

    assert!(!codec.strict());
    assert!(!Leb128Codec::default().strict());

    codec.set_strict(true);

    assert!(codec.strict());
    assert!(Leb128Codec::with_strict(true).strict());
}

#[test]
fn test_unsigned_leb128_round_trips_fixed_width_values() {
    let codec = Leb128Codec::new();
    let mut output = [0_u8; 64];
    let mut offset = 0;

    offset += codec
        .write_uleb_u16_at(&mut output, offset, 300)
        .expect("u16 should fit");
    offset += codec
        .write_uleb_u32_at(&mut output, offset, 0x1f600)
        .expect("u32 should fit");
    offset += codec
        .write_uleb_u64_at(&mut output, offset, 0x0102_0304_0506_0708)
        .expect("u64 should fit");
    offset += codec
        .write_uleb_u128_at(
            &mut output,
            offset,
            0x0102_0304_0506_0708_1112_1314_1516_1718,
        )
        .expect("u128 should fit");

    let mut index = 0;
    let (value, consumed) = codec
        .read_uleb_u16_at(&output[..offset], index)
        .expect("valid u16")
        .expect("complete u16");
    assert_eq!(300, value);
    index += consumed;

    let (value, consumed) = codec
        .read_uleb_u32_at(&output[..offset], index)
        .expect("valid u32")
        .expect("complete u32");
    assert_eq!(0x1f600, value);
    index += consumed;

    let (value, consumed) = codec
        .read_uleb_u64_at(&output[..offset], index)
        .expect("valid u64")
        .expect("complete u64");
    assert_eq!(0x0102_0304_0506_0708, value);
    index += consumed;

    let (value, consumed) = codec
        .read_uleb_u128_at(&output[..offset], index)
        .expect("valid u128")
        .expect("complete u128");
    assert_eq!(0x0102_0304_0506_0708_1112_1314_1516_1718, value);
    index += consumed;

    assert_eq!(offset, index);
}

#[test]
fn test_signed_leb128_round_trips_fixed_width_values() {
    let codec = Leb128Codec::new();
    let mut output = [0_u8; 64];
    let mut offset = 0;

    offset += codec
        .write_sleb_i16_at(&mut output, offset, -300)
        .expect("i16 should fit");
    offset += codec
        .write_sleb_i32_at(&mut output, offset, -0x1f600)
        .expect("i32 should fit");
    offset += codec
        .write_sleb_i64_at(&mut output, offset, -0x0102_0304_0506_0708)
        .expect("i64 should fit");
    offset += codec
        .write_sleb_i128_at(
            &mut output,
            offset,
            -0x0102_0304_0506_0708_1112_1314_1516_1718,
        )
        .expect("i128 should fit");

    let mut index = 0;
    let (value, consumed) = codec
        .read_sleb_i16_at(&output[..offset], index)
        .expect("valid i16")
        .expect("complete i16");
    assert_eq!(-300, value);
    index += consumed;

    let (value, consumed) = codec
        .read_sleb_i32_at(&output[..offset], index)
        .expect("valid i32")
        .expect("complete i32");
    assert_eq!(-0x1f600, value);
    index += consumed;

    let (value, consumed) = codec
        .read_sleb_i64_at(&output[..offset], index)
        .expect("valid i64")
        .expect("complete i64");
    assert_eq!(-0x0102_0304_0506_0708, value);
    index += consumed;

    let (value, consumed) = codec
        .read_sleb_i128_at(&output[..offset], index)
        .expect("valid i128")
        .expect("complete i128");
    assert_eq!(-0x0102_0304_0506_0708_1112_1314_1516_1718, value);
    index += consumed;

    assert_eq!(offset, index);
}

#[test]
fn test_read_returns_none_for_incomplete_values() {
    let codec = Leb128Codec::new();

    assert_eq!(None, codec.read_uleb_u16_at(&[], 0).expect("empty input"));
    assert_eq!(
        None,
        codec
            .read_uleb_u16_at(&[0x80], 0)
            .expect("truncated unsigned input"),
    );
    assert_eq!(
        None,
        codec
            .read_sleb_i16_at(&[0x80], 0)
            .expect("truncated signed input"),
    );
}

#[test]
fn test_read_rejects_out_of_range_and_noncanonical_values() {
    let codec = Leb128Codec::new();
    let error = codec
        .read_uleb_u16_at(&[0], 2)
        .expect_err("out-of-range index should fail");
    assert_eq!(2, error.index());
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());
    assert_eq!("malformed LEB128 integer", error.to_string());

    let error = codec
        .read_sleb_i16_at(&[0], 2)
        .expect_err("out-of-range signed index should fail");
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());

    let error = codec
        .read_uleb_u16_at(&[0x80, 0x80, 0x80, 0x80, 0x00], 0)
        .expect_err("too-wide u16 should fail");
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());
    let error = codec
        .read_uleb_u16_at(&[0x80, 0x80, 0x04], 0)
        .expect_err("too-wide final unsigned payload should fail");
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());
    let error = codec
        .read_sleb_i16_at(&[0x80, 0x80, 0x04], 0)
        .expect_err("too-wide final signed payload should fail");
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());
    let error = codec
        .read_sleb_i16_at(&[0x80, 0x80, 0x80], 0)
        .expect_err("unterminated signed value should fail");
    assert_eq!(Leb128DecodeErrorKind::Malformed, error.kind());

    let strict = Leb128Codec::with_strict(true);
    let error = strict
        .read_uleb_u16_at(&[0x80, 0x00], 0)
        .expect_err("non-canonical unsigned value should fail");
    assert_eq!(Leb128DecodeErrorKind::NonCanonical, error.kind());

    let error = strict
        .read_sleb_i16_at(&[0xff, 0x7f], 0)
        .expect_err("non-canonical signed value should fail");
    assert_eq!(Leb128DecodeErrorKind::NonCanonical, error.kind());
    assert_eq!(
        Leb128DecodeError::new(Leb128DecodeErrorKind::NonCanonical, 0),
        error,
    );
}

#[test]
fn test_write_returns_none_when_output_is_too_small() {
    let codec = Leb128Codec::new();
    let mut output = [0_u8; 1];

    assert_eq!(None, codec.write_uleb_u16_at(&mut output, 0, 300));
    assert_eq!(None, codec.write_sleb_i16_at(&mut output, 0, -300));
    assert_eq!(None, codec.write_uleb_u128_at(&mut output, usize::MAX, 0));
}
