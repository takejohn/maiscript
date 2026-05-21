use syntax::{CodePoint, EsString};

use crate::lexical::{char_stream::PeekableStream, code_points, token::TokenContent};

/// This function assumes that the input stream starts with a quotation mark.
pub(super) fn read_string_literal(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> TokenContent {
	enum State { String, Escape }

	let mut value = EsString::new();
	let literal_mark = stream.next().expect("expected quotation mark");
	assert!(literal_mark == code_points::QUOTATION_MARK || literal_mark == code_points::APOSTROPHE);
	let mut state = State::String;

	for c in stream {
		match state {
			State::String => {
				if c == code_points::REVERSE_SOLIDUS {
					state = State::Escape;
				} else if c == literal_mark {
					return TokenContent::StringLiteral(value);
				} else {
					value.push_code_point(c);
				}
			},

			State::Escape => {
				value.push_code_point(c);
				state = State::String;
			},
		}
	}

	return TokenContent::IncompleteStringLiteral;
}

#[cfg(test)]
mod tests {
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
		let token = read_string_literal(&mut stream);
		return MatchResult { token, stream };
	}

	#[test]
	fn single_quote_in_double_quote_string() {
		let expected = TokenContent::StringLiteral(EsString::from("'"));
		assert_eq!(to_token(r#""'""#).matches(), expected);
	}

	#[test]
	fn double_quote_in_single_quote_string() {
		let expected = TokenContent::StringLiteral(EsString::from("\""));
		assert_eq!(to_token(r#"'"'"#).matches(), expected);
	}

	#[test]
	fn incomplete_double_quote_string() {
		let expected = TokenContent::IncompleteStringLiteral;
		assert_eq!(to_token(r#""abc"#).matches(), expected);
	}

	#[test]
	fn incomplete_single_quote_string() {
		let expected = TokenContent::IncompleteStringLiteral;
		assert_eq!(to_token(r#"'abc"#).matches(), expected);
	}

	#[test]
	fn escaped_double_quote() {
		let expected = TokenContent::StringLiteral(EsString::from("\""));
		assert_eq!(to_token(r#""\"""#).matches(), expected);
	}

	#[test]
	fn escaped_single_quote() {
		let expected = TokenContent::StringLiteral(EsString::from("\'"));
		assert_eq!(to_token(r"'\''").matches(), expected);
	}

	#[test]
	fn incomplete_escape() {
		let expected = TokenContent::IncompleteStringLiteral;
		assert_eq!(to_token(r#""\"#).matches(), expected);
	}
}
