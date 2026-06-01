use std::borrow::Cow;

use maiscript_char_stream::CharStream;
use maiscript_string::{CodePoint, EsStr, EsString};

use crate::lexical::{
    code_points,
    match_token::{
        identifier_name::try_read_identifier_name, number_literal::try_read_number_literal,
        string_literal::read_string_literal,
    },
    token::TokenContent,
};

mod identifier_name;
mod number_literal;
mod string_literal;

const NUMBER_SIGN_2_ES_STR: &EsStr = EsStr::from_utf16(&['#' as u16, '#' as u16]);
const AMPERSAND_ES_STR: &EsStr = EsStr::from_utf16(&['&' as u16]);

/// This function assumes the input stream starts with a token.
/// Input stream must not start with a space or comment.
pub(super) fn match_token(stream: &mut CharStream<'_>) -> TokenContent {
    match stream.char() {
        None => {
            stream.next();
            TokenContent::EOF
        }
        Some(code_points::LINE_FEED) => {
            stream.next();
            TokenContent::NewLine
        }
        Some(code_points::EXCLAMATION_MARK) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::NotEq
                }
                _ => {
                    stream.next();
                    TokenContent::Not
                }
            }
        }
        Some(code_points::QUOTATION_MARK) | Some(code_points::APOSTROPHE) => {
            read_string_literal(stream)
        }
        Some(code_points::NUMBER_SIGN) => {
            stream.next();
            match stream.char() {
                Some(code_points::NUMBER_SIGN) => {
                    stream.next();
                    match stream.char() {
                        Some(code_points::NUMBER_SIGN) => {
                            stream.next();
                            TokenContent::Sharp3
                        }
                        _ => {
                            stream.next();
                            TokenContent::Unknown(Cow::Borrowed(NUMBER_SIGN_2_ES_STR))
                        }
                    }
                }
                Some(code_points::LEFT_SQUARE_BRACKET) => {
                    stream.next();
                    TokenContent::OpenSharpBracket
                }
                _ => {
                    stream.next();
                    TokenContent::Sharp
                }
            }
        }
        Some(code_points::PERCENT_SIGN) => {
            stream.next();
            TokenContent::Percent
        }
        Some(code_points::AMPERSAND) => {
            stream.next();
            match stream.char() {
                Some(code_points::AMPERSAND) => {
                    stream.next();
                    TokenContent::And2
                }
                _ => {
                    stream.next();
                    TokenContent::Unknown(Cow::Borrowed(AMPERSAND_ES_STR))
                }
            }
        }
        Some(code_points::LEFT_PARENTHESIS) => {
            stream.next();
            TokenContent::OpenParen
        }
        Some(code_points::RIGHT_PARENTHESIS) => {
            stream.next();
            TokenContent::CloseParen
        }
        Some(code_points::ASTERISK) => {
            stream.next();
            TokenContent::Asterisk
        }
        Some(code_points::PLUS_SIGN) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::PlusEq
                }
                _ => {
                    stream.next();
                    TokenContent::Plus
                }
            }
        }
        Some(code_points::COMMA) => {
            stream.next();
            TokenContent::Comma
        }
        Some(code_points::HYPHEN_MINUS) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::MinusEq
                }
                _ => {
                    stream.next();
                    TokenContent::Minus
                }
            }
        }
        Some(code_points::FULL_STOP) => {
            stream.next();
            TokenContent::Dot
        }
        Some(code_points::SOLIDUS) => {
            stream.next();
            TokenContent::Slash
        }
        Some(code_points::COLON) => {
            stream.next();
            match stream.char() {
                Some(code_points::COLON) => {
                    stream.next();
                    TokenContent::Colon2
                }
                _ => {
                    stream.next();
                    TokenContent::Colon
                }
            }
        }
        Some(code_points::SEMICOLON) => {
            stream.next();
            TokenContent::SemiColon
        }
        Some(code_points::LESS_THAN_SIGN) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::LtEq
                }
                Some(code_points::COLON) => {
                    stream.next();
                    TokenContent::Out
                }
                _ => {
                    stream.next();
                    TokenContent::Lt
                }
            }
        }
        Some(code_points::EQUALS_SIGN) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::Eq2
                }
                Some(code_points::GREATER_THAN_SIGN) => {
                    stream.next();
                    TokenContent::Arrow
                }
                _ => {
                    stream.next();
                    TokenContent::Eq
                }
            }
        }
        Some(code_points::GREATER_THAN_SIGN) => {
            stream.next();
            match stream.char() {
                Some(code_points::EQUALS_SIGN) => {
                    stream.next();
                    TokenContent::GtEq
                }
                _ => {
                    stream.next();
                    TokenContent::Gt
                }
            }
        }
        Some(code_points::QUESTION_MARK) => {
            stream.next();
            TokenContent::Question
        }
        Some(code_points::COMMERCIAL_AT) => {
            stream.next();
            TokenContent::At
        }
        Some(code_points::LEFT_SQUARE_BRACKET) => {
            stream.next();
            TokenContent::OpenBracket
        }
        Some(code_points::REVERSE_SOLIDUS) => {
            if stream.char_nth(1) == Some(CodePoint::from_char('u')) {
                todo!()
            } else {
                stream.next();
                TokenContent::BackSlash
            }
        }
        Some(code_points::RIGHT_SQUARE_BRACKET) => {
            stream.next();
            TokenContent::CloseBracket
        }
        Some(code_points::CIRCUMFLEX_ACCENT) => {
            stream.next();
            TokenContent::Hat
        }
        Some(code_points::LEFT_CURLY_BRACKET) => {
            stream.next();
            TokenContent::OpenBrace
        }
        Some(code_points::VERTICAL_LINE) => {
            stream.next();
            match stream.char() {
                Some(code_points::VERTICAL_LINE) => {
                    stream.next();
                    TokenContent::Or2
                }
                _ => {
                    stream.next();
                    TokenContent::Or
                }
            }
        }
        Some(code_points::RIGHT_CURLY_BRACKET) => {
            stream.next();
            TokenContent::CloseBrace
        }
        Some(cp) => {
            if let Some(digit_token) = try_read_number_literal(stream) {
                return digit_token;
            };
            if let Some(word_token) = try_read_identifier_name(stream) {
                return word_token;
            }
            return TokenContent::Unknown(Cow::Owned(EsString::from_utf16(
                cp.encode_utf16().to_vec(),
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use maiscript_string::EsString;

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

    fn to_token(source: &EsStr) -> MatchResult<'_> {
        let mut stream = CharStream::new(&source);
        let token = match_token(&mut stream);
        return MatchResult { token, stream };
    }

    #[test]
    fn token_eof() {
        assert_eq!(to_token(&EsString::from("")).matches(), TokenContent::EOF);
    }

    #[test]
    fn token_new_line() {
        assert_eq!(
            to_token(&EsString::from("\n")).matches(),
            TokenContent::NewLine
        );
    }

    #[test]
    fn token_not() {
        assert_eq!(to_token(&EsString::from("!")).matches(), TokenContent::Not);
    }

    #[test]
    fn token_not_eq() {
        assert_eq!(
            to_token(&EsString::from("!=")).matches(),
            TokenContent::NotEq
        );
    }

    #[test]
    fn double_quote_string_literal() {
        assert_eq!(
            to_token(&EsString::from(r#""abc""#)).matches(),
            TokenContent::StringLiteral(EsString::from("abc"))
        );
    }

    #[test]
    fn single_quote_string_literal() {
        assert_eq!(
            to_token(&EsString::from("'abc'")).matches(),
            TokenContent::StringLiteral(EsString::from("abc"))
        );
    }

    #[test]
    fn incomplete_string_literal() {
        assert_eq!(
            to_token(&EsString::from("\"")).matches(),
            TokenContent::IncompleteStringLiteral
        );
    }

    #[test]
    fn token_sharp() {
        assert_eq!(
            to_token(&EsString::from("#")).matches(),
            TokenContent::Sharp
        );
    }

    #[test]
    fn token_open_sharp_bracket() {
        assert_eq!(
            to_token(&EsString::from("#[")).matches(),
            TokenContent::OpenSharpBracket
        );
    }

    #[test]
    fn token_sharp3() {
        assert_eq!(
            to_token(&EsString::from("###")).matches(),
            TokenContent::Sharp3
        );
    }

    #[test]
    fn unknown_sharp2() {
        assert_eq!(
            to_token(&EsString::from("##")).matches(),
            TokenContent::Unknown(Cow::Borrowed(NUMBER_SIGN_2_ES_STR))
        );
    }

    #[test]
    fn token_percent() {
        assert_eq!(
            to_token(&EsString::from("%")).matches(),
            TokenContent::Percent
        );
    }

    #[test]
    fn token_and2() {
        assert_eq!(
            to_token(&EsString::from("&&")).matches(),
            TokenContent::And2
        );
    }

    #[test]
    fn unknown_and() {
        assert_eq!(
            to_token(&EsString::from("&")).matches(),
            TokenContent::Unknown(Cow::Borrowed(AMPERSAND_ES_STR))
        );
    }

    #[test]
    fn token_open_paren() {
        assert_eq!(
            to_token(&EsString::from("(")).matches(),
            TokenContent::OpenParen
        );
    }

    #[test]
    fn token_close_paren() {
        assert_eq!(
            to_token(&EsString::from(")")).matches(),
            TokenContent::CloseParen
        );
    }

    #[test]
    fn token_asterisk() {
        assert_eq!(
            to_token(&EsString::from("*")).matches(),
            TokenContent::Asterisk
        );
    }

    #[test]
    fn token_plus() {
        assert_eq!(to_token(&EsString::from("+")).matches(), TokenContent::Plus);
    }

    #[test]
    fn token_plus_eq() {
        assert_eq!(
            to_token(&EsString::from("+=")).matches(),
            TokenContent::PlusEq
        );
    }

    #[test]
    fn token_comma() {
        assert_eq!(
            to_token(&EsString::from(",")).matches(),
            TokenContent::Comma
        );
    }

    #[test]
    fn token_minus() {
        assert_eq!(
            to_token(&EsString::from("-")).matches(),
            TokenContent::Minus
        );
    }

    #[test]
    fn token_minus_eq() {
        assert_eq!(
            to_token(&EsString::from("-=")).matches(),
            TokenContent::MinusEq
        );
    }

    #[test]
    fn token_dot() {
        assert_eq!(to_token(&EsString::from(".")).matches(), TokenContent::Dot);
    }

    #[test]
    fn token_slash() {
        assert_eq!(
            to_token(&EsString::from("/")).matches(),
            TokenContent::Slash
        );
    }

    #[test]
    fn token_colon() {
        assert_eq!(
            to_token(&EsString::from(":")).matches(),
            TokenContent::Colon
        );
    }

    #[test]
    fn token_colon2() {
        assert_eq!(
            to_token(&EsString::from("::")).matches(),
            TokenContent::Colon2
        );
    }

    #[test]
    fn token_semi_colon() {
        assert_eq!(
            to_token(&EsString::from(";")).matches(),
            TokenContent::SemiColon
        );
    }

    #[test]
    fn token_lt() {
        assert_eq!(to_token(&EsString::from("<")).matches(), TokenContent::Lt);
    }

    #[test]
    fn token_lt_eq() {
        assert_eq!(
            to_token(&EsString::from("<=")).matches(),
            TokenContent::LtEq
        );
    }

    #[test]
    fn token_out() {
        assert_eq!(to_token(&EsString::from("<:")).matches(), TokenContent::Out);
    }

    #[test]
    fn token_eq() {
        assert_eq!(to_token(&EsString::from("=")).matches(), TokenContent::Eq);
    }

    #[test]
    fn token_eq2() {
        assert_eq!(to_token(&EsString::from("==")).matches(), TokenContent::Eq2);
    }

    #[test]
    fn token_arrow() {
        assert_eq!(
            to_token(&EsString::from("=>")).matches(),
            TokenContent::Arrow
        );
    }

    #[test]
    fn token_gt() {
        assert_eq!(to_token(&EsString::from(">")).matches(), TokenContent::Gt);
    }

    #[test]
    fn token_gt_eq() {
        assert_eq!(
            to_token(&EsString::from(">=")).matches(),
            TokenContent::GtEq
        );
    }

    #[test]
    fn token_question() {
        assert_eq!(
            to_token(&EsString::from("?")).matches(),
            TokenContent::Question
        );
    }

    #[test]
    fn token_at() {
        assert_eq!(to_token(&EsString::from("@")).matches(), TokenContent::At);
    }

    #[test]
    fn token_open_bracket() {
        assert_eq!(
            to_token(&EsString::from("[")).matches(),
            TokenContent::OpenBracket
        );
    }

    #[test]
    fn token_back_slash() {
        assert_eq!(
            to_token(&EsString::from("\\")).matches(),
            TokenContent::BackSlash
        );
    }

    #[test]
    fn token_close_bracket() {
        assert_eq!(
            to_token(&EsString::from("]")).matches(),
            TokenContent::CloseBracket
        );
    }

    #[test]
    fn token_hat() {
        assert_eq!(to_token(&EsString::from("^")).matches(), TokenContent::Hat);
    }

    #[test]
    fn token_open_brace() {
        assert_eq!(
            to_token(&EsString::from("{")).matches(),
            TokenContent::OpenBrace
        );
    }

    #[test]
    fn token_or() {
        assert_eq!(to_token(&EsString::from("|")).matches(), TokenContent::Or);
    }

    #[test]
    fn token_or2() {
        assert_eq!(to_token(&EsString::from("||")).matches(), TokenContent::Or2);
    }

    #[test]
    fn token_close_brace() {
        assert_eq!(
            to_token(&EsString::from("}")).matches(),
            TokenContent::CloseBrace
        );
    }

    #[test]
    fn token_number_literal() {
        assert_eq!(
            to_token(&EsString::from("123.456")).matches(),
            TokenContent::NumberLiteral(EsString::from("123.456"))
        );
    }

    #[test]
    fn token_incomplete_number_literal() {
        assert_eq!(
            to_token(&EsString::from("123.")).matches(),
            TokenContent::IncompleteNumberLiteral(EsString::from("123."))
        );
    }

    #[test]
    fn token_identifier_name() {
        assert_eq!(
            to_token(&EsString::from("if")).matches(),
            TokenContent::IdentifierName(EsString::from("if"))
        );
    }
}
