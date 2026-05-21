use maiscript_syntax::{CodePoint, EsString};

use crate::lexical::{char_stream::PeekableStream, code_points, token::TokenContent};

pub(super) fn read_number_literal(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> Option<TokenContent> {
	let mut value = EsString::new();

	read_digits(stream, &mut value)?;

	if let Some(decimal_point) = stream.next_if_eq(code_points::FULL_STOP) {
		value.push_code_point(decimal_point);
		if read_digits(stream, &mut value).is_none() {
			return Some(TokenContent::IncompleteNumberLiteral(value));
		}
	}

	return Some(TokenContent::NumberLiteral(value));
}

fn read_digits(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>, dst: &mut EsString) -> Option<()> {
	let first = stream.next_if(code_points::is_digit)?;
	dst.push_code_point(first);
	while let Some(c) = stream.next_if(code_points::is_digit) {
		dst.push_code_point(c);
	}
	Some(())
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MatchResult<I> where I: Iterator<Item = CodePoint> {
		token: Option<TokenContent>,
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
		let token = read_number_literal(&mut stream);
		return MatchResult { token, stream };
	}

	#[test]
	fn integer() {
		assert_eq!(to_token("123").matches(), TokenContent::NumberLiteral(EsString::from("123")));
	}

	#[test]
	fn fraction() {
		assert_eq!(to_token("0.456").matches(), TokenContent::NumberLiteral(EsString::from("0.456")));
	}

	#[test]
	fn lacking_fraction_part() {
		assert_eq!(to_token("123.").matches(), TokenContent::IncompleteNumberLiteral(EsString::from("123.")));
	}

	#[test]
	fn lacking_integer_part() {
		let MatchResult { token, mut stream } = to_token(".123");
		assert_eq!(token, None);
		assert_eq!(stream.peek(0), Some(code_points::FULL_STOP));
	}
}
