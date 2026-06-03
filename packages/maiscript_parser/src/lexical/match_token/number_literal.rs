use boa_string::CommonJsStringBuilder;
use maiscript_char_stream::CharStream;

use crate::{
    lexical::{code_points, token::TokenContent},
    utils::PushCodePoint,
};

pub(super) fn try_read_number_literal(stream: &mut CharStream<'_>) -> Option<TokenContent> {
    let mut builder = CommonJsStringBuilder::new();

    try_read_digits(stream, &mut builder)?;

    if let Some(decimal_point) = stream.next_if_eq(code_points::FULL_STOP) {
        builder.push_code_point(decimal_point);
        if try_read_digits(stream, &mut builder).is_none() {
            return Some(TokenContent::IncompleteNumberLiteral(builder.build()));
        }
    }

    return Some(TokenContent::NumberLiteral(builder.build()));
}

fn try_read_digits(stream: &mut CharStream<'_>, dst: &mut CommonJsStringBuilder) -> Option<()> {
    let first = stream.next_if(code_points::is_digit)?;
    dst.push_code_point(first);
    while let Some(c) = stream.next_if(code_points::is_digit) {
        dst.push_code_point(c);
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use boa_string::JsStr;
    use boa_string_literal::js_str;
    use maiscript_char_stream::CharStream;

    use super::*;

    struct MatchResult<'a> {
        token: Option<TokenContent>,
        stream: CharStream<'a>,
    }

    impl MatchResult<'_> {
        fn matches(self) -> TokenContent {
            assert!(self.stream.char().is_none());
            self.token.unwrap()
        }
    }

    fn to_token(source: JsStr<'_>) -> MatchResult<'_> {
        let mut stream = CharStream::new(source);
        let token = try_read_number_literal(&mut stream);
        return MatchResult { token, stream };
    }

    #[test]
    fn integer() {
        assert_eq!(
            to_token(js_str!("123")).matches(),
            TokenContent::NumberLiteral(js_str!("123").into())
        );
    }

    #[test]
    fn fraction() {
        assert_eq!(
            to_token(js_str!("0.456")).matches(),
            TokenContent::NumberLiteral(js_str!("0.456").into())
        );
    }

    #[test]
    fn lacking_fraction_part() {
        assert_eq!(
            to_token(js_str!("123.")).matches(),
            TokenContent::IncompleteNumberLiteral(js_str!("123.").into())
        );
    }

    #[test]
    fn lacking_integer_part() {
        let source = js_str!(".123");
        let MatchResult { token, stream } = to_token(source);
        assert_eq!(token, None);
        assert_eq!(stream.char(), Some(code_points::FULL_STOP));
    }
}
