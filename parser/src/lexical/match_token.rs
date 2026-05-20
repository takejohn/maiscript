use std::borrow::Cow;

use syntax::{CodePoint, EsStr};

use crate::lexical::{char_stream::PeekableStream, code_points, match_token::string_literal::read_string_literal, token::TokenContent};

mod string_literal;

const NUMBER_SIGN_2_ES_STR: &EsStr = EsStr::from_u16s(&['#' as u16, '#' as u16]);
const AMPERSAND_ES_STR: &EsStr = EsStr::from_u16s(&['&' as u16]);

/// This function assumes the input stream starts with a token.
/// Input stream must not start with a space or comment.
pub(super) fn match_token(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> TokenContent {
	match stream.peek(0) {
		None => { stream.next(); TokenContent::EOF },
		Some(code_points::LINE_FEED) => { stream.next(); TokenContent::NewLine },
		Some(code_points::EXCLAMATION_MARK) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::NotEq },
			_ => { stream.next(); TokenContent::Not }
		} },
		Some(code_points::QUOTATION_MARK) | Some(code_points::APOSTROPHE) => read_string_literal(stream),
		Some(code_points::NUMBER_SIGN) => { stream.next(); match stream.peek(0) {
			Some(code_points::NUMBER_SIGN) => { stream.next(); match stream.peek(0) {
				Some(code_points::NUMBER_SIGN) => { stream.next(); TokenContent::Sharp3 },
				_ => { stream.next(); TokenContent::Unknown(Cow::Borrowed(NUMBER_SIGN_2_ES_STR)) },
			} },
			Some(code_points::LEFT_SQUARE_BRACKET) => { stream.next(); TokenContent::OpenSharpBracket },
			_ => { stream.next(); TokenContent::Sharp },
		} },
		Some(code_points::PERCENT_SIGN) => { stream.next(); TokenContent::Percent },
		Some(code_points::AMPERSAND) => { stream.next(); match stream.peek(0) {
			Some(code_points::AMPERSAND) => { stream.next(); TokenContent::And2 },
			_ => { stream.next(); TokenContent::Unknown(Cow::Borrowed(AMPERSAND_ES_STR)) },
		} },
		Some(code_points::LEFT_PARENTHESIS) => { stream.next(); TokenContent::OpenParen },
		Some(code_points::RIGHT_PARENTHESIS) => { stream.next(); TokenContent::CloseParen },
		Some(code_points::ASTERISK) => { stream.next(); TokenContent::Asterisk },
		Some(code_points::PLUS_SIGN) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::PlusEq },
			_ => { stream.next(); TokenContent::Plus },
		} },
		Some(code_points::COMMA) => { stream.next(); TokenContent::Comma },
		Some(code_points::HYPHEN_MINUS) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::MinusEq },
			_ => { stream.next(); TokenContent::Minus },
		} },
		Some(code_points::FULL_STOP) => { stream.next(); TokenContent::Dot },
		Some(code_points::SOLIDUS) => { stream.next(); TokenContent::Slash },
		Some(code_points::COLON) => { stream.next(); match stream.peek(0) {
			Some(code_points::COLON) => { stream.next(); TokenContent::Colon2 },
			_ => { stream.next(); TokenContent::Colon }
		} },
		Some(code_points::SEMICOLON) => { stream.next(); TokenContent::SemiColon },
		Some(code_points::LESS_THAN_SIGN) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::LtEq },
			Some(code_points::COLON) => { stream.next(); TokenContent::Out },
			_ => { stream.next(); TokenContent::Lt },
		} },
		Some(code_points::EQUALS_SIGN) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::Eq2 },
			Some(code_points::GREATER_THAN_SIGN) => { stream.next(); TokenContent::Arrow },
			_ => { stream.next(); TokenContent::Eq },
		} },
		Some(code_points::GREATER_THAN_SIGN) => { stream.next(); match stream.peek(0) {
			Some(code_points::EQUALS_SIGN) => { stream.next(); TokenContent::GtEq },
			_ => { stream.next(); TokenContent::Gt },
		} },
		Some(code_points::QUESTION_MARK) => { stream.next(); TokenContent::Question },
		Some(code_points::COMMERCIAL_AT) => { stream.next(); TokenContent::At },
		Some(code_points::LEFT_SQUARE_BRACKET) => { stream.next(); TokenContent::OpenBracket },
		Some(code_points::REVERSE_SOLIDUS) => if stream.peek(1) == Some(CodePoint::from_char('u')) {
			todo!()
		} else {
			stream.next();
			TokenContent::BackSlash
		}
		Some(code_points::RIGHT_SQUARE_BRACKET) => { stream.next(); TokenContent::CloseBracket },
		Some(code_points::CIRCUMFLEX_ACCENT) => { stream.next(); TokenContent::Hat },
		Some(code_points::LEFT_CURLY_BRACKET) => { stream.next(); TokenContent::OpenBrace },
		Some(code_points::VERTICAL_LINE) => { stream.next(); match stream.peek(0) {
			Some(code_points::VERTICAL_LINE) => { stream.next(); TokenContent::Or2 },
			_ => { stream.next(); TokenContent::Or },
		} },
		Some(code_points::RIGHT_CURLY_BRACKET) => { stream.next(); TokenContent::CloseBrace },
		Some(_) => {
			todo!()
		},
	}
}

#[cfg(test)]
mod tests {
	use syntax::EsString;

	use super::*;

	struct MatchResult<I> where I: Iterator<Item = CodePoint> {
		token: TokenContent,
		stream: PeekableStream<I>
	}

	impl<I> MatchResult<I> where I: Iterator<Item = CodePoint> {
		fn matches(self) -> TokenContent {
			let mut stream = self.stream;
			assert!(stream.peek(0).is_none());
			self.token
		}
	}

	fn to_token(content: &str) -> MatchResult<impl Iterator<Item = CodePoint>> {
		let source = content.chars().map(CodePoint::from_char);
		let mut stream = PeekableStream::new(source);
		let token = match_token(&mut stream);
		return MatchResult { token, stream };
	}

	#[test]
	fn token_eof() {
		assert_eq!(to_token("").matches(), TokenContent::EOF);
	}

	#[test]
	fn token_new_line() {
		assert_eq!(to_token("\n").matches(), TokenContent::NewLine);
	}

	#[test]
	fn token_not() {
		assert_eq!(to_token("!").matches(), TokenContent::Not);
	}

	#[test]
	fn token_not_eq() {
		assert_eq!(to_token("!=").matches(), TokenContent::NotEq);
	}

	#[test]
	fn double_quote_string_literal() {
		assert_eq!(to_token(r#""abc""#).matches(), TokenContent::StringLiteral(EsString::from("abc")));
	}

	#[test]
	fn single_quote_string_literal() {
		assert_eq!(to_token("'abc'").matches(), TokenContent::StringLiteral(EsString::from("abc")));
	}

	#[test]
	fn incomplete_string_literal() {
		assert_eq!(to_token("\"").matches(), TokenContent::IncompleteStringLiteral);
	}

	#[test]
	fn token_sharp() {
		assert_eq!(to_token("#").matches(), TokenContent::Sharp);
	}

	#[test]
	fn token_open_sharp_bracket() {
		assert_eq!(to_token("#[").matches(), TokenContent::OpenSharpBracket);
	}

	#[test]
	fn token_sharp3() {
		assert_eq!(to_token("###").matches(), TokenContent::Sharp3);
	}

	#[test]
	fn unknown_sharp2() {
		assert_eq!(to_token("##").matches(), TokenContent::Unknown(Cow::Borrowed(NUMBER_SIGN_2_ES_STR)));
	}

	#[test]
	fn token_percent() {
		assert_eq!(to_token("%").matches(), TokenContent::Percent);
	}

	#[test]
	fn token_and2() {
		assert_eq!(to_token("&&").matches(), TokenContent::And2);
	}

	#[test]
	fn unknown_and() {
		assert_eq!(to_token("&").matches(), TokenContent::Unknown(Cow::Borrowed(AMPERSAND_ES_STR)));
	}

	#[test]
	fn token_open_paren() {
		assert_eq!(to_token("(").matches(), TokenContent::OpenParen);
	}

	#[test]
	fn token_close_paren() {
		assert_eq!(to_token(")").matches(), TokenContent::CloseParen);
	}

	#[test]
	fn token_asterisk() {
		assert_eq!(to_token("*").matches(), TokenContent::Asterisk);
	}

	#[test]
	fn token_plus() {
		assert_eq!(to_token("+").matches(), TokenContent::Plus);
	}

	#[test]
	fn token_plus_eq() {
		assert_eq!(to_token("+=").matches(), TokenContent::PlusEq);
	}

	#[test]
	fn token_comma() {
		assert_eq!(to_token(",").matches(), TokenContent::Comma);
	}

	#[test]
	fn token_minus() {
		assert_eq!(to_token("-").matches(), TokenContent::Minus);
	}

	#[test]
	fn token_minus_eq() {
		assert_eq!(to_token("-=").matches(), TokenContent::MinusEq);
	}

	#[test]
	fn token_dot() {
		assert_eq!(to_token(".").matches(), TokenContent::Dot);
	}

	#[test]
	fn token_slash() {
		assert_eq!(to_token("/").matches(), TokenContent::Slash);
	}

	#[test]
	fn token_colon() {
		assert_eq!(to_token(":").matches(), TokenContent::Colon);
	}

	#[test]
	fn token_colon2() {
		assert_eq!(to_token("::").matches(), TokenContent::Colon2);
	}

	#[test]
	fn token_semi_colon() {
		assert_eq!(to_token(";").matches(), TokenContent::SemiColon);
	}

	#[test]
	fn token_lt() {
		assert_eq!(to_token("<").matches(), TokenContent::Lt);
	}

	#[test]
	fn token_lt_eq() {
		assert_eq!(to_token("<=").matches(), TokenContent::LtEq);
	}

	#[test]
	fn token_out() {
		assert_eq!(to_token("<:").matches(), TokenContent::Out);
	}

	#[test]
	fn token_eq() {
		assert_eq!(to_token("=").matches(), TokenContent::Eq);
	}

	#[test]
	fn token_eq2() {
		assert_eq!(to_token("==").matches(), TokenContent::Eq2);
	}

	#[test]
	fn token_arrow() {
		assert_eq!(to_token("=>").matches(), TokenContent::Arrow);
	}

	#[test]
	fn token_gt() {
		assert_eq!(to_token(">").matches(), TokenContent::Gt);
	}

	#[test]
	fn token_gt_eq() {
		assert_eq!(to_token(">=").matches(), TokenContent::GtEq);
	}

	#[test]
	fn token_question() {
		assert_eq!(to_token("?").matches(), TokenContent::Question);
	}

	#[test]
	fn token_at() {
		assert_eq!(to_token("@").matches(), TokenContent::At);
	}

	#[test]
	fn token_open_bracket() {
		assert_eq!(to_token("[").matches(), TokenContent::OpenBracket);
	}

	#[test]
	fn token_back_slash() {
		assert_eq!(to_token("\\").matches(), TokenContent::BackSlash);
	}

	#[test]
	fn token_close_bracket() {
		assert_eq!(to_token("]").matches(), TokenContent::CloseBracket);
	}

	#[test]
	fn token_hat() {
		assert_eq!(to_token("^").matches(), TokenContent::Hat);
	}

	#[test]
	fn token_open_brace() {
		assert_eq!(to_token("{").matches(), TokenContent::OpenBrace);
	}

	#[test]
	fn token_or() {
		assert_eq!(to_token("|").matches(), TokenContent::Or);
	}

	#[test]
	fn token_or2() {
		assert_eq!(to_token("||").matches(), TokenContent::Or2);
	}

	#[test]
	fn token_close_brace() {
		assert_eq!(to_token("}").matches(), TokenContent::CloseBrace);
	}
}
