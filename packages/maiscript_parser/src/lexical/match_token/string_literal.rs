use boa_string::{CodePoint, CommonJsStringBuilder};
use maiscript_char_stream::CharStream;

use crate::{
    lexical::{code_points, token::TokenContent},
    utils::PushCodePoint,
};

/// This function assumes that the input stream starts with a quotation mark.
pub(super) fn read_string_literal(stream: &mut CharStream<'_>) -> TokenContent {
    enum State {
        String,
        Escape,
    }

    let mut builder = CommonJsStringBuilder::new();
    let literal_mark = stream.next().expect("expected quotation mark");
    assert!(literal_mark == code_points::QUOTATION_MARK || literal_mark == code_points::APOSTROPHE);
    let mut state = State::String;

    for c in stream {
        match state {
            State::String => {
                if c == code_points::REVERSE_SOLIDUS {
                    state = State::Escape;
                } else if c == literal_mark {
                    return TokenContent::StringLiteral(builder.build());
                } else {
                    builder.push_code_point(c);
                }
            }

            State::Escape => {
                builder.push_code_point(c);
                state = State::String;
            }
        }
    }

    return TokenContent::IncompleteStringLiteral;
}

#[cfg(test)]
mod tests {
    use boa_string::JsStr;
    use boa_string_literal::js_str;

    use super::*;

    struct MatchResult<'a> {
        token: TokenContent,
        stream: CharStream<'a>,
    }

    impl MatchResult<'_> {
        fn matches(self) -> TokenContent {
            assert!(self.stream.char().is_none());
            self.token
        }
    }

    fn to_token(source: JsStr<'_>) -> MatchResult<'_> {
        let mut stream = CharStream::new(source);
        let token = read_string_literal(&mut stream);
        return MatchResult { token, stream };
    }

    #[test]
    fn single_quote_in_double_quote_string() {
        let expected = TokenContent::StringLiteral(js_str!("'").into());
        assert_eq!(to_token(js_str!(r#""'""#)).matches(), expected);
    }

    #[test]
    fn double_quote_in_single_quote_string() {
        let expected = TokenContent::StringLiteral(js_str!("\"").into());
        assert_eq!(to_token(js_str!(r#"'"'"#)).matches(), expected);
    }

    #[test]
    fn incomplete_double_quote_string() {
        let expected = TokenContent::IncompleteStringLiteral;
        assert_eq!(to_token(js_str!(r#""abc"#)).matches(), expected);
    }

    #[test]
    fn incomplete_single_quote_string() {
        let expected = TokenContent::IncompleteStringLiteral;
        assert_eq!(to_token(js_str!(r#"'abc"#)).matches(), expected);
    }

    #[test]
    fn escaped_double_quote() {
        let expected = TokenContent::StringLiteral(js_str!("\"").into());
        assert_eq!(to_token(js_str!(r#""\"""#)).matches(), expected);
    }

    #[test]
    fn escaped_single_quote() {
        let expected = TokenContent::StringLiteral(js_str!("\'").into());
        assert_eq!(to_token(js_str!(r"'\''")).matches(), expected);
    }

    #[test]
    fn incomplete_escape() {
        let expected = TokenContent::IncompleteStringLiteral;
        assert_eq!(to_token(js_str!(r#""\"#)).matches(), expected);
    }
}
