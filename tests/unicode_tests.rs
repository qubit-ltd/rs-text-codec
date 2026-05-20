use qubit_unicode::Unicode;

#[test]
fn test_unicode_validates_code_point_ranges() {
    assert!(Unicode::is_valid_ascii(0x7f));
    assert!(!Unicode::is_valid_ascii(0x80));
    assert!(!Unicode::is_valid_ascii(-1));

    assert!(Unicode::is_valid_latin1(0xff));
    assert!(!Unicode::is_valid_latin1(0x100));

    assert!(Unicode::is_valid_unicode(0x10ffff));
    assert!(!Unicode::is_valid_unicode(0x110000));
    assert!(!Unicode::is_valid_unicode(-1));

    assert!(Unicode::is_bmp(0xffff));
    assert!(!Unicode::is_bmp(0x10000));
    assert!(Unicode::is_supplementary(0x10000));
    assert!(Unicode::is_supplementary(0x10ffff));
    assert!(!Unicode::is_supplementary(0x110000));
}

#[test]
fn test_unicode_handles_surrogate_pairs() {
    let code_point = 0x1f600;
    let high = Unicode::decompose_high_surrogate(code_point).expect("valid supplementary point");
    let low = Unicode::decompose_low_surrogate(code_point).expect("valid supplementary point");

    assert_eq!(0xd83d, high);
    assert_eq!(0xde00, low);
    assert!(Unicode::is_high_surrogate(high as i32));
    assert!(Unicode::is_low_surrogate(low as i32));
    assert!(Unicode::is_surrogate(high as i32));
    assert!(Unicode::is_surrogate_pair(high, low));
    assert_eq!(Some(code_point), Unicode::compose_surrogate_pair(high, low));
    assert_eq!(None, Unicode::compose_surrogate_pair(low, high));
    assert_eq!(None, Unicode::decompose_high_surrogate('A' as u32));
    assert_eq!(None, Unicode::decompose_low_surrogate('A' as u32));
}

#[test]
fn test_unicode_escapes_code_points_like_java_unicode_escape() {
    assert_eq!(Some("\\u0041".to_string()), Unicode::escape('A' as u32));
    assert_eq!(Some("\\u4E2D".to_string()), Unicode::escape('中' as u32));
    assert_eq!(Some("\\u1F600".to_string()), Unicode::escape(0x1f600));
    assert_eq!(None, Unicode::escape(0x110000));
    assert_eq!(Some(0), Unicode::plane('A' as u32));
    assert_eq!(Some(1), Unicode::plane(0x1f600));
    assert_eq!(None, Unicode::plane(0x110000));
}
