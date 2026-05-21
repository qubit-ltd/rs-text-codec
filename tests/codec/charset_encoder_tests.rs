use qubit_text_codec::{
    Charset,
    CharsetCodec,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetEncodeError,
    CharsetEncodeErrorKind,
    CharsetEncodeResult,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    UnmappableAction,
};

#[derive(Clone, Copy, Debug, Default)]
struct AsciiBytesCodec;

impl CharsetCodec<u8> for AsciiBytesCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        if index > input.len() {
            return Err(qubit_text_codec::CharsetDecodeError::malformed_sequence(
                Charset::ASCII,
                index,
            ));
        }
        if index == input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: 0,
            });
        }
        let value = input[index];
        if value > 0x7f {
            return Err(qubit_text_codec::CharsetDecodeError::malformed_sequence(
                Charset::ASCII,
                index,
            ));
        }
        Ok(DecodeStatus::Complete {
            value: value as char,
            consumed: 1,
        })
    }

    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if !ch.is_ascii() {
            return Err(CharsetEncodeError::unmappable_character(
                Charset::ASCII,
                index,
                ch as u32,
            ));
        }
        if index >= output.len() {
            return Err(CharsetEncodeError::buffer_too_small(Charset::ASCII, index));
        }
        output[index] = ch as u8;
        Ok(1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct InvalidBangCodec;

impl CharsetCodec<u8> for InvalidBangCodec {
    fn charset(&self) -> Charset {
        Charset::ASCII
    }

    fn max_units_per_char(&self) -> usize {
        1
    }

    fn decode_one(&self, _input: &[u8], index: usize) -> CharsetDecodeResult<DecodeStatus> {
        Err(CharsetDecodeError::malformed_sequence(
            Charset::ASCII,
            index,
        ))
    }

    fn encode_one(&self, ch: char, output: &mut [u8], index: usize) -> CharsetEncodeResult<usize> {
        if ch == '!' {
            return Err(CharsetEncodeError::invalid_code_point(
                Charset::ASCII,
                index,
                ch as u32,
            ));
        }
        if index >= output.len() {
            return Err(CharsetEncodeError::buffer_too_small(Charset::ASCII, index));
        }
        output[index] = ch as u8;
        Ok(1)
    }
}

#[test]
fn test_charset_encoder_exposes_configuration_and_bounds() {
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    assert_eq!(Charset::ASCII, encoder.codec().charset());
    assert_eq!(Charset::ASCII, encoder.codec_mut().charset());
    assert_eq!(UnmappableAction::Replace, encoder.unmappable_action());
    assert_eq!('?', encoder.replacement());
    assert_eq!(Some(3), encoder.max_output_len(3));

    encoder.set_replacement('*');
    encoder.set_unmappable_action(UnmappableAction::Ignore);

    assert_eq!('*', encoder.replacement());
    assert_eq!(UnmappableAction::Ignore, encoder.unmappable_action());
}

#[test]
fn test_charset_encoder_replaces_reports_and_ignores_unmappable_input() {
    let input = ['A', 'é', 'B'];
    let mut output = [0_u8; 3];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let progress = encoder
        .convert(&input, 0, &mut output, 0)
        .expect("default unmappable action replaces");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(3, progress.written());
    assert_eq!(b"A?B", &output);

    encoder.set_unmappable_action(UnmappableAction::Ignore);
    let mut ignored_output = [0_u8; 2];
    let progress = encoder
        .convert(&input, 0, &mut ignored_output, 0)
        .expect("ignore unmappable input");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(b"AB", &ignored_output);

    encoder.set_unmappable_action(UnmappableAction::Report);
    let error = encoder
        .convert(&input, 1, &mut output, 0)
        .expect_err("report unmappable input");

    assert_eq!(CharsetEncodeErrorKind::UnmappableCharacter, error.kind());
    assert_eq!(1, error.index());
    assert_eq!(Some('é' as u32), error.value());
}

#[test]
fn test_charset_encoder_reports_need_output_when_replacement_does_not_fit() {
    let input = ['A', 'é'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let progress = encoder
        .convert(&input, 0, &mut output, 0)
        .expect("small output should stop with NeedOutput");

    assert_eq!(CoderStatus::NeedOutput, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!(b"A", &output);
}

#[test]
fn test_charset_encoder_reports_invalid_indices_and_capacity() {
    let input = ['A', 'B'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);

    let error = encoder
        .convert(&input, input.len() + 1, &mut output, 0)
        .expect_err("input index is outside input slice");
    assert_eq!(CharsetEncodeErrorKind::InvalidInputIndex, error.kind());
    assert_eq!(input.len() + 1, error.index());

    let beyond_output = output.len() + 1;
    let progress = encoder
        .convert(&input, 0, &mut output, beyond_output)
        .expect("output index beyond output slice needs more output");
    assert_eq!(CoderStatus::NeedOutput, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let progress = encoder
        .convert(&input, 0, &mut output, 0)
        .expect("normal encoding stops when output fills");
    assert_eq!(CoderStatus::NeedOutput, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(1, progress.written());
}

#[test]
fn test_charset_encoder_reports_unmappable_replacement() {
    let input = ['中'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    encoder.set_replacement('é');

    let error = encoder
        .convert(&input, 0, &mut output, 0)
        .expect_err("replacement character is unmappable too");

    assert_eq!(CharsetEncodeErrorKind::UnmappableCharacter, error.kind());
    assert_eq!(0, error.index());
    assert_eq!(Some('é' as u32), error.value());
}

#[test]
fn test_charset_encoder_propagates_non_policy_encoding_errors() {
    let input = ['!'];
    let mut output = [0_u8; 1];
    let mut encoder = CharsetEncoder::new(InvalidBangCodec);

    let error = encoder
        .convert(&input, 0, &mut output, 0)
        .expect_err("invalid code point error is not absorbed");

    assert_eq!(CharsetEncodeErrorKind::InvalidCodePoint, error.kind());
    assert_eq!(Some('!' as u32), error.value());
}
