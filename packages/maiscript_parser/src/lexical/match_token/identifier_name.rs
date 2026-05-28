use maiscript_string::{CodePoint, EsString};
use unicode_id_start::{is_id_continue, is_id_start};

use crate::lexical::{char_stream::PeekableStream, code_points, token::TokenContent};

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
pub(super) fn try_read_identifier_name(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> Option<TokenContent> {
    let s = read_identifier_name_string(stream);
    (!s.is_empty()).then_some(TokenContent::IdentifierName(s))
}

pub(super) fn read_identifier_name_string(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> EsString {
    let mut result = EsString::new();
    if !read_unicode_escape_sequence_or(stream, &mut result, is_id_start) {
        return result;
    }
    loop {
        if !read_unicode_escape_sequence_or(stream, &mut result, is_id_continue) {
            return result;
        }
    }
}

/// Returns true if it is able to read the rest part of `IdentifierName`, otherwise false.
fn read_unicode_escape_sequence_or(
    stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>,
    buf: &mut EsString,
    predicate: impl FnOnce(char) -> bool,
) -> bool {
    let Some(next) = stream.peek(0) else {
        return false;
    };

    if next == code_points::REVERSE_SOLIDUS {
        return read_unicode_escape_sequence(stream, buf);
    }

    if next.as_char().is_some_and(predicate) {
        stream.next();
        buf.push_code_point(next);
        return true
    }

    return false;
}

/// Reads a unicode escape sequence.
/// Assumes that the stream to start with a reverse solidus.
/// Returns true if it is able to read the rest part of `IdentifierName`, otherwise false.
fn read_unicode_escape_sequence(
    stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>,
    buf: &mut EsString,
) -> bool {
    if stream.peek(1).is_none_or(|cp| cp != CodePoint::from_char('u')) {
        return false;
    }

    stream.next();
    buf.push_code_point(code_points::REVERSE_SOLIDUS);
    stream.next();
    buf.push_code_point(CodePoint::from_char('u'));

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
    use super::*;

    struct MatchIdentifierName<I> where I: Iterator<Item = CodePoint> {
        token: Option<TokenContent>,
        stream: PeekableStream<I>,
    }

    impl<I> MatchIdentifierName<I> where I: Iterator<Item = CodePoint> {
        fn matches_exact(self) -> TokenContent {
            let mut stream = self.stream;
            assert!(stream.peek(0).is_none());
            self.token.unwrap()
        }

        fn matches(self) -> (TokenContent, EsString) {
            let token = self.token.unwrap();
            let rest = EsString::from_code_points(self.stream);
            (token, rest)
        }
    }

    fn match_identifier_name(s: &str) -> MatchIdentifierName<impl Iterator<Item = CodePoint>> {
        let mut stream = PeekableStream::new(CodePoint::decode_utf16(s.encode_utf16()));
        let token = try_read_identifier_name(&mut stream);
        return MatchIdentifierName { token, stream };
    }

    #[test]
    fn ascii_ident() {
        assert_eq!(match_identifier_name("abc").matches_exact(), TokenContent::IdentifierName(EsString::from("abc")));
    }

    #[test]
    fn keyword_if() {
        assert_eq!(match_identifier_name("if").matches_exact(), TokenContent::IdentifierName(EsString::from("if")));
    }

    #[test]
    fn unicode_escape_sequence_4_digits() {
        assert_eq!(match_identifier_name(r#"\u0041"#).matches_exact(), TokenContent::IdentifierName(EsString::from(r#"\u0041"#)));
    }

    #[test]
    fn unicode_escape_sequence_incomplete_continue() {
        assert_eq!(
            match_identifier_name(r#"\u004 "#).matches(),
            (TokenContent::IdentifierName(EsString::from(r#"\u004"#)), EsString::from(" ")),
        );
    }

    #[test]
    fn unicode_escape_sequence_incomplete_eof() {
        assert_eq!(match_identifier_name(r#"\u004"#).matches_exact(), TokenContent::IdentifierName(EsString::from(r#"\u004"#)));
    }

    #[test]
    fn unicode_escape_sequence_bracket() {
        assert_eq!(match_identifier_name(r#"\u{000041}"#).matches_exact(), TokenContent::IdentifierName(EsString::from(r#"\u{000041}"#)));
    }

    #[test]
    fn unicode_escape_sequence_bracket_not_closed() {
        assert_eq!(match_identifier_name(r#"\u{000041"#).matches_exact(), TokenContent::IdentifierName(EsString::from(r#"\u{000041"#)));
    }

    #[test]
    fn unicode_escape_sequence_bracket_with_space() {
        assert_eq!(match_identifier_name(r#"\u{000041 }"#).matches_exact(), TokenContent::IdentifierName(EsString::from(r#"\u{000041 }"#)));
    }

    #[test]
    fn invalid_sequence() {
        assert_eq!(match_identifier_name(r#"\x41"#).token, None);
    }
}
