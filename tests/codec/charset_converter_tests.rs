use qubit_text_codec::{
    Charset,
    CharsetCodec,
    CharsetConvertError,
    CharsetConverter,
    CharsetDecodeError,
    CharsetDecodeResult,
    CharsetDecoder,
    CharsetEncodeError,
    CharsetEncodeResult,
    CharsetEncoder,
    Coder,
    CoderStatus,
    DecodeStatus,
    MalformedAction,
    UnmappableAction,
    Utf8Codec,
    Utf16U16Codec,
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
        if index >= input.len() {
            return Ok(DecodeStatus::NeedMore {
                required: index + 1,
                available: input.len().saturating_sub(index),
            });
        }
        let value = input[index];
        if value > 0x7f {
            return Err(CharsetDecodeError::malformed_sequence(
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

#[test]
fn test_charset_converter_exposes_configuration_and_bounds() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);

    assert_eq!(Charset::UTF_8, converter.decoder().codec().charset());
    assert_eq!(Charset::UTF_16, converter.encoder().codec().charset());
    assert_eq!(Some(6), converter.max_output_len(3));

    converter
        .decoder_mut()
        .set_malformed_action(MalformedAction::Ignore);
    converter
        .encoder_mut()
        .set_unmappable_action(UnmappableAction::Ignore);

    assert_eq!(
        MalformedAction::Ignore,
        converter.decoder().malformed_action()
    );
    assert_eq!(
        UnmappableAction::Ignore,
        converter.encoder().unmappable_action()
    );

    converter.reset();
}

#[test]
fn test_charset_converter_combines_decoder_and_encoder_with_offsets() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let input = "A中".as_bytes();
    let mut output = [0_u16; 1];

    let progress = converter
        .convert(input, 1, &mut output, 0)
        .expect("UTF-8 to UTF-16 conversion");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(3, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('中' as u16, output[0]);
}

#[test]
fn test_charset_converter_keeps_pending_character_when_output_is_full() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut empty_output = [];

    let progress = converter
        .convert(b"A", 0, &mut empty_output, 0)
        .expect("decoded character stays pending");

    assert_eq!(CoderStatus::NeedOutput, progress.status());
    assert_eq!(1, progress.read());
    assert_eq!(0, progress.written());

    let progress = converter
        .convert(b"", 0, &mut empty_output, 0)
        .expect("pending character still needs output");

    assert_eq!(CoderStatus::NeedOutput, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());

    let mut output = [0_u16; 1];
    let progress = converter
        .convert(b"", 0, &mut output, 0)
        .expect("pending character is written before reading more input");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(1, progress.written());
    assert_eq!('A' as u16, output[0]);
}

#[test]
fn test_charset_converter_continues_after_decoder_fills_char_buffer() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 2];

    let progress = converter
        .convert(b"AB", 0, &mut output, 0)
        .expect("converter loops after decoder reports char-buffer output full");

    assert_eq!(CoderStatus::Complete, progress.status());
    assert_eq!(2, progress.read());
    assert_eq!(2, progress.written());
    assert_eq!(['A' as u16, 'B' as u16], output);
}

#[test]
fn test_charset_converter_reports_need_input_from_decoder() {
    let decoder = CharsetDecoder::new(Utf8Codec);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 1];

    let progress = converter
        .convert(&[0xe4], 0, &mut output, 0)
        .expect("partial source sequence needs more input");

    assert_eq!(CoderStatus::NeedInput, progress.status());
    assert_eq!(0, progress.read());
    assert_eq!(0, progress.written());
}

#[test]
fn test_charset_converter_propagates_decode_and_encode_errors() {
    let mut decoder = CharsetDecoder::new(Utf8Codec);
    decoder.set_malformed_action(MalformedAction::Report);
    let encoder = CharsetEncoder::new(Utf16U16Codec);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut output = [0_u16; 1];

    let error = converter
        .convert(&[0x80], 0, &mut output, 0)
        .expect_err("malformed source input is reported");
    assert!(matches!(error, CharsetConvertError::Decode(_)));

    let decoder = CharsetDecoder::new(Utf8Codec);
    let mut encoder = CharsetEncoder::new(AsciiBytesCodec);
    encoder.set_unmappable_action(UnmappableAction::Report);
    let mut converter = CharsetConverter::new(decoder, encoder);
    let mut ascii_output = [0_u8; 1];

    let error = converter
        .convert("é".as_bytes(), 0, &mut ascii_output, 0)
        .expect_err("unmappable target character is reported");
    assert!(matches!(error, CharsetConvertError::Encode(_)));
}
