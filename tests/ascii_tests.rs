use qubit_unicode::Ascii;

#[test]
fn test_ascii_classifies_ascii_code_points() {
    assert!(Ascii::is_ascii_byte(b'A'));
    assert!(!Ascii::is_ascii_byte(0x80));
    assert!(Ascii::is_ascii_char('~'));
    assert!(!Ascii::is_ascii_char('中'));
    assert!(Ascii::is_ascii_code_point(0x7f));
    assert!(!Ascii::is_ascii_code_point(-1));
    assert!(!Ascii::is_ascii_code_point(0x80));
}

#[test]
fn test_ascii_classifies_common_character_sets() {
    assert!(Ascii::is_whitespace('\n' as i32));
    assert!(Ascii::is_whitespace(' ' as i32));
    assert!(!Ascii::is_whitespace('\u{00a0}' as i32));

    assert!(Ascii::is_letter('Q' as i32));
    assert!(Ascii::is_upper_case_letter('Q' as i32));
    assert!(Ascii::is_lower_case_letter('q' as i32));
    assert!(Ascii::is_digit('7' as i32));
    assert!(Ascii::is_hex_digit('f' as i32));
    assert!(Ascii::is_hex_digit('F' as i32));
    assert!(Ascii::is_octal_digit('7' as i32));
    assert!(!Ascii::is_octal_digit('8' as i32));
    assert!(Ascii::is_letter_or_digit('9' as i32));
    assert!(Ascii::is_printable('~' as i32));
    assert!(Ascii::is_control(0x1f));
}

#[test]
fn test_ascii_converts_case_and_digits() {
    assert!(Ascii::equals_ignore_case('A' as i32, 'a' as i32));
    assert!(Ascii::equals_ignore_case('A' as i32, 'A' as i32));
    assert!(!Ascii::equals_ignore_case('A' as i32, 'B' as i32));

    assert_eq!(b'Q', Ascii::to_upper_case_byte(b'q'));
    assert_eq!(b'Q', Ascii::to_upper_case_byte(b'Q'));
    assert_eq!('Q', Ascii::to_upper_case_char('q'));
    assert_eq!('Q', Ascii::to_upper_case_char('Q'));
    assert_eq!('Q' as i32, Ascii::to_upper_case_code_point('q' as i32));
    assert_eq!('Q' as i32, Ascii::to_upper_case_code_point('Q' as i32));
    assert_eq!(b'q', Ascii::to_lower_case_byte(b'Q'));
    assert_eq!(b'q', Ascii::to_lower_case_byte(b'q'));
    assert_eq!('q', Ascii::to_lower_case_char('Q'));
    assert_eq!('q', Ascii::to_lower_case_char('q'));
    assert_eq!('q' as i32, Ascii::to_lower_case_code_point('Q' as i32));

    assert_eq!(Some(7), Ascii::to_digit('7' as i32));
    assert_eq!(None, Ascii::to_digit('x' as i32));
    assert_eq!(Some(9), Ascii::to_hex_digit('9' as i32));
    assert_eq!(Some(15), Ascii::to_hex_digit('F' as i32));
    assert_eq!(Some(15), Ascii::to_hex_digit('f' as i32));
    assert_eq!(None, Ascii::to_hex_digit('x' as i32));
}

#[test]
fn test_ascii_fold_matches_java_ascii_fold_examples() {
    let mut buffer = ['\0'; Ascii::MAX_FOLDING];

    let count = Ascii::fold('Æ', &mut buffer, 0);
    assert_eq!(2, count);
    assert_eq!(&['A', 'E'], &buffer[..count]);

    let count = Ascii::fold('⒑', &mut buffer, 0);
    assert_eq!(3, count);
    assert_eq!(&['1', '0', '.'], &buffer[..count]);

    let count = Ascii::fold('⑽', &mut buffer, 0);
    assert_eq!(4, count);
    assert_eq!(&['(', '1', '0', ')'], &buffer[..count]);

    let count = Ascii::fold('中', &mut buffer, 0);
    assert_eq!(1, count);
    assert_eq!('中', buffer[0]);

    let count = Ascii::fold('A', &mut buffer, 0);
    assert_eq!(1, count);
    assert_eq!('A', buffer[0]);

    assert_eq!("AE", Ascii::fold_to_string('Æ'));
}
