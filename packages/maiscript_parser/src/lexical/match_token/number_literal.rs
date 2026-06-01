use maiscript_char_stream::CharStream;
use maiscript_string::EsString;

use crate::lexical::{code_points, token::TokenContent};

pub(super) fn try_read_number_literal(stream: &mut CharStream<'_>) -> Option<TokenContent> {
    let mut value = EsString::new();

    try_read_digits(stream, &mut value)?;

    if let Some(decimal_point) = stream.next_if_eq(code_points::FULL_STOP) {
        value.push_code_point(decimal_point);
        if try_read_digits(stream, &mut value).is_none() {
            return Some(TokenContent::IncompleteNumberLiteral(value));
        }
    }

    return Some(TokenContent::NumberLiteral(value));
}

fn try_read_digits(stream: &mut CharStream<'_>, dst: &mut EsString) -> Option<()> {
    let first = stream.next_if(code_points::is_digit)?;
    dst.push_code_point(first);
    while let Some(c) = stream.next_if(code_points::is_digit) {
        dst.push_code_point(c);
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use maiscript_char_stream::CharStream;
    use maiscript_string::EsStr;

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

    fn to_token(source: &EsStr) -> MatchResult<'_> {
        let mut stream = CharStream::new(&source);
        let token = try_read_number_literal(&mut stream);
        return MatchResult { token, stream };
    }

    #[test]
    fn integer() {
        assert_eq!(
            to_token(&EsString::from("123")).matches(),
            TokenContent::NumberLiteral(EsString::from("123"))
        );
    }

    #[test]
    fn fraction() {
        assert_eq!(
            to_token(&EsString::from("0.456")).matches(),
            TokenContent::NumberLiteral(EsString::from("0.456"))
        );
    }

    #[test]
    fn lacking_fraction_part() {
        assert_eq!(
            to_token(&EsString::from("123.")).matches(),
            TokenContent::IncompleteNumberLiteral(EsString::from("123."))
        );
    }

    #[test]
    fn lacking_integer_part() {
        let source = EsString::from(".123");
        let MatchResult { token, stream } = to_token(&source);
        assert_eq!(token, None);
        assert_eq!(stream.char(), Some(code_points::FULL_STOP));
    }
}
