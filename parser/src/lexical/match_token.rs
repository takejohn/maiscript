use syntax::{CodePoint, EsString, code_point_of};

use crate::{error::{AiScriptSyntaxError, AiScriptSyntaxErrorSource, Result}, lexical::{char_stream::PeekableStream, code_points::{self, LEFT_SQUARE_BRACKET}, skip_space::SkipSpace, token::TokenContent}};

/// This function assumes the input stream starts with a token.
/// Input stream must not start with a space or comment.
pub(super) fn match_token(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> Result<TokenContent> {
	macro_rules! trie {
		{ $( $pat:pat => $expr:expr ),* $(,)? } => {
			match stream.peek(0) {
				$(
					$pat => {
						stream.next();
						$expr
					}
				),*
			}
		};
	}

	if stream.peek(0).is_some_and(code_points::is_line_break_char) {
		stream.skip_whitespace_and_comments()?;
		return Ok(TokenContent::NewLine);
	}

	let pos = stream.get_pos().clone();
	let err = |source: AiScriptSyntaxErrorSource| {
		Result::<TokenContent>::Err(AiScriptSyntaxError { source, pos })
	};

	trie!{
		None => Ok(TokenContent::EOF),
		Some(code_points::EXCLAMATION_MARK) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::NotEq),
			_ => Ok(TokenContent::Not)
		},
		Some(code_points::QUOTATION_MARK) | Some(code_points::APOSTROPHE) => todo!(),
		Some(code_points::NUMBER_SIGN) => trie!{
			Some(code_points::NUMBER_SIGN) => trie!{
				Some(code_points::NUMBER_SIGN) => Ok(TokenContent::Sharp3),
				_ => err(AiScriptSyntaxErrorSource::InvalidCharacterSequence(
					EsString::from_char_code(['#' as u16, '#' as u16]),
				)),
			},
			Some(code_points::LEFT_SQUARE_BRACKET) => Ok(TokenContent::OpenSharpBracket),
			_ => Ok(TokenContent::Sharp),
		},
		Some(code_points::PERCENT_SIGN) => Ok(TokenContent::Percent),
		Some(code_points::AMPERSAND) => trie!{
			Some(code_points::AMPERSAND) => Ok(TokenContent::And2),
			_ => err(AiScriptSyntaxErrorSource::InvalidCharacter(code_points::AMPERSAND)),
		},
		Some(code_points::LEFT_PARENTHESIS) => Ok(TokenContent::OpenParen),
		Some(code_points::RIGHT_PARENTHESIS) => Ok(TokenContent::CloseParen),
		Some(code_points::ASTERISK) => Ok(TokenContent::Asterisk),
		Some(code_points::PLUS_SIGN) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::PlusEq),
			_ => Ok(TokenContent::Plus),
		},
		Some(code_points::COMMA) => Ok(TokenContent::Comma),
		Some(code_points::HYPHEN_MINUS) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::MinusEq),
			_ => Ok(TokenContent::Minus),
		},
		Some(code_points::FULL_STOP) => Ok(TokenContent::Dot),
		Some(code_points::SOLIDUS) => Ok(TokenContent::Slash),
		Some(code_points::COLON) => trie!{
			Some(code_points::COLON) => Ok(TokenContent::Colon2),
			_ => Ok(TokenContent::Colon)
		},
		Some(code_points::SEMICOLON) => Ok(TokenContent::SemiColon),
		Some(code_points::LESS_THAN_SIGN) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::LtEq),
			Some(code_points::COLON) => Ok(TokenContent::Out),
			_ => Ok(TokenContent::Lt),
		},
		Some(code_points::EQUALS_SIGN) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::Eq2),
			Some(code_points::GREATER_THAN_SIGN) => Ok(TokenContent::Arrow),
			_ => Ok(TokenContent::Eq),
		},
		Some(code_points::GREATER_THAN_SIGN) => trie!{
			Some(code_points::EQUALS_SIGN) => Ok(TokenContent::GtEq),
			_ => Ok(TokenContent::Gt),
		},
		Some(code_points::QUESTION_MARK) => Ok(TokenContent::Question),
		Some(code_points::COMMERCIAL_AT) => Ok(TokenContent::At),
		Some(code_points::LEFT_SQUARE_BRACKET) => Ok(TokenContent::OpenBracket),
		Some(code_points::RIGHT_SQUARE_BRACKET) => Ok(TokenContent::CloseBracket),
		Some(code_points::CIRCUMFLEX_ACCENT) => Ok(TokenContent::Hat),
		Some(code_points::LEFT_CURLY_BRACKET) => Ok(TokenContent::OpenBrace),
		Some(code_points::VERTICAL_LINE) => trie!{
			Some(code_points::VERTICAL_LINE) => Ok(TokenContent::Or2),
			_ => Ok(TokenContent::Or),
		},
		Some(code_points::RIGHT_CURLY_BRACKET) => Ok(TokenContent::CloseBrace),
		Some(start_char) => {
			if start_char == code_points::REVERSE_SOLIDUS {
				if stream.peek(1) == Some(code_point_of!('u')) {
					todo!()
				} else {
					stream.next();
					Ok(TokenContent::BackSlash)
				}
			} else {
				todo!()
			}
		},
	}
}

#[cfg(test)]
mod tests {
	use syntax::Position;

use super::*;

	struct MatchResult<I> where I: Iterator<Item = CodePoint> {
		token: Result<TokenContent>,
		stream: PeekableStream<I>
	}

	impl<I> MatchResult<I> where I: Iterator<Item = CodePoint> {
		fn matches(self) -> TokenContent {
			let mut stream = self.stream;
			assert!(stream.peek(0).is_none());
			self.token.unwrap()
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
	fn error_sharp2() {
		let Err(err) = to_token("##").token else {
			unreachable!();
		};
		let AiScriptSyntaxError {
			source: AiScriptSyntaxErrorSource::InvalidCharacterSequence(source),
			pos
		} = err else {
			unreachable!()
		};
		assert_eq!(source, EsString::from_char_code(['#' as u16, '#' as u16]));
		assert_eq!(pos, Position::ZERO);
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
	fn error_and() {
		let Err(err) = to_token("&").token else {
			unreachable!();
		};
		let AiScriptSyntaxError {
			source: AiScriptSyntaxErrorSource::InvalidCharacter(source),
			pos
		} = err else {
			unreachable!()
		};
		assert_eq!(source, code_point_of!('&'));
		assert_eq!(pos, Position::ZERO);
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
