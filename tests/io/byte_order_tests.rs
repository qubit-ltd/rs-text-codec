use qubit_text_codec::ByteOrder;

#[test]
fn test_variants_are_distinct_values() {
    assert_ne!(ByteOrder::BigEndian, ByteOrder::LittleEndian);
    assert_eq!(ByteOrder::BigEndian, ByteOrder::BigEndian);
}
