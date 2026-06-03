use boa_string::{CodePoint, CommonJsStringBuilder, JsString};
use maiscript_char_stream::CharStream;
use unicode_id_start::{is_id_continue, is_id_start};

use crate::{
    lexical::{code_points, token::TokenContent},
    utils::PushCodePoint,
};

/// Reads an `IdentifierName`.
///
/// The `IdentifierName` starts with a Unicode code point which has the property ID_START or a unicode escape sequence.
/// Every rest part is a Unicode code point which has the property ID_CONTINUE or a unicode escape sequence.
/// A unicode escape sequence consists of a reverse solidus, letter 'u', and some trailing characters.
/// The trailing characters can be any length if they are enclosed by curly brackets, otherwise up to 4 long.
/// If the trailing character are not enclosed by brackets, it cannot contain character except code points with the property ID_CONTINUE.
///
/// Consequently, this function allows an ECMAScript IdentifierName, which is a superset of AiScript `Identifier` and keywords.
/// Additionally, some unicode escape sequences which are invalid in ECMAScript are allowed.
pub(super) fn try_read_identifier_name(stream: &mut CharStream<'_>) -> Option<TokenContent> {
    let s = read_identifier_name_string(stream);
    (!s.is_empty()).then_some(TokenContent::IdentifierName(s))
}

pub(super) fn read_identifier_name_string(stream: &mut CharStream<'_>) -> JsString {
    let mut builder = CommonJsStringBuilder::new();
    if !read_unicode_escape_sequence_or(stream, &mut builder, is_id_start) {
        return builder.build();
    }
    loop {
        if !read_unicode_escape_sequence_or(stream, &mut builder, is_id_continue) {
            return builder.build();
        }
    }
}

/// Returns true if it is able to read the rest part of `IdentifierName`, otherwise false.
fn read_unicode_escape_sequence_or(
    stream: &mut CharStream<'_>,
    buf: &mut CommonJsStringBuilder,
    predicate: impl FnOnce(char) -> bool,
) -> bool {
    let Some(next) = stream.char() else {
        return false;
    };

    if next == code_points::REVERSE_SOLIDUS {
        return read_unicode_escape_sequence(stream, buf);
    }

    if next.as_char().is_some_and(predicate) {
        stream.next();
        buf.push_code_point(next);
        return true;
    }

    return false;
}

/// Reads a unicode escape sequence.
/// Assumes that the stream to start with a reverse solidus.
/// Returns true if it is able to read the rest part of `IdentifierName`, otherwise false.
fn read_unicode_escape_sequence(
    stream: &mut CharStream<'_>,
    buf: &mut CommonJsStringBuilder,
) -> bool {
    if stream
        .char_nth(1)
        .is_none_or(|cp| cp != CodePoint::Unicode('u'))
    {
        return false;
    }

    stream.next();
    buf.push_code_point(code_points::REVERSE_SOLIDUS);
    stream.next();
    buf.push_code_point(CodePoint::Unicode('u'));

    if stream.next_if_eq(code_points::LEFT_CURLY_BRACKET).is_some() {
        buf.push_code_point(code_points::LEFT_CURLY_BRACKET);
        while let Some(cp) = stream.next() {
            buf.push_code_point(cp);
            if cp == code_points::RIGHT_CURLY_BRACKET {
                return true;
            }
        }
        // reached EOF
        return false;
    }

    for _ in 0..4 {
        let Some(cp) = stream.next_if(|cp| cp.as_char().is_some_and(is_id_continue)) else {
            return false;
        };
        buf.push_code_point(cp);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use boa_string::{JsStr, JsString};
    use boa_string_literal::js_str;

    use super::*;

    struct MatchIdentifierName<'a> {
        token: Option<TokenContent>,
        stream: CharStream<'a>,
    }

    impl<'a> MatchIdentifierName<'a> {
        fn matches_exact(self) -> TokenContent {
            assert!(self.stream.char().is_none());
            self.token.unwrap()
        }

        fn matches(self) -> (TokenContent, CharStream<'a>) {
            let token = self.token.unwrap();
            (token, self.stream)
        }
    }

    fn match_identifier_name(s: JsStr) -> MatchIdentifierName<'_> {
        let mut stream = CharStream::new(s);
        let token = try_read_identifier_name(&mut stream);
        return MatchIdentifierName { token, stream };
    }

    #[test]
    fn ascii_ident() {
        assert_eq!(
            match_identifier_name(js_str!("abc")).matches_exact(),
            TokenContent::IdentifierName(js_str!("abc").into())
        );
    }

    #[test]
    fn keyword_if() {
        assert_eq!(
            match_identifier_name(js_str!("if")).matches_exact(),
            TokenContent::IdentifierName(js_str!("if").into())
        );
    }

    #[test]
    fn unicode_escape_sequence_4_digits() {
        assert_eq!(
            match_identifier_name(js_str!(r#"\u0041"#)).matches_exact(),
            TokenContent::IdentifierName(js_str!(r#"\u0041"#).into())
        );
    }

    #[test]
    fn unicode_escape_sequence_incomplete_continue() {
        let (token, mut stream) = match_identifier_name(js_str!(r#"\u004 "#)).matches();
        assert_eq!(
            token,
            TokenContent::IdentifierName(js_str!(r#"\u004"#).into())
        );
        assert_eq!(stream.next(), Some(CodePoint::Unicode(' ')));
        assert_eq!(stream.next(), None)
    }

    #[test]
    fn unicode_escape_sequence_incomplete_eof() {
        assert_eq!(
            match_identifier_name(js_str!(r#"\u004"#)).matches_exact(),
            TokenContent::IdentifierName(js_str!(r#"\u004"#).into())
        );
    }

    #[test]
    fn unicode_escape_sequence_bracket() {
        assert_eq!(
            match_identifier_name(js_str!(r#"\u{000041}"#)).matches_exact(),
            TokenContent::IdentifierName(js_str!(r#"\u{000041}"#).into())
        );
    }

    #[test]
    fn unicode_escape_sequence_bracket_not_closed() {
        assert_eq!(
            match_identifier_name(js_str!(r#"\u{000041"#)).matches_exact(),
            TokenContent::IdentifierName(js_str!(r#"\u{000041"#).into())
        );
    }

    #[test]
    fn unicode_escape_sequence_bracket_with_space() {
        assert_eq!(
            match_identifier_name(js_str!(r#"\u{000041 }"#)).matches_exact(),
            TokenContent::IdentifierName(js_str!(r#"\u{000041 }"#).into())
        );
    }

    #[test]
    fn invalid_sequence() {
        assert_eq!(match_identifier_name(js_str!(r#"\x41"#)).token, None);
    }
}
