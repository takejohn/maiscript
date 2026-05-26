use maiscript_string::CodePoint;
use maiscript_syntax::Position;

#[derive(Debug)]
/// Iterator of CodePoints with get_pos method which can be used to get current position.
pub(crate) struct PosStream<I> where I: Iterator<Item = CodePoint> {
	inner: I,
	pos: Position,
}

impl<I> PosStream<I> where I: Iterator<Item = CodePoint> {
	pub(super) fn new(inner: impl IntoIterator<IntoIter = I>) -> Self {
		Self {
			inner: inner.into_iter(),
			pos: Position::ZERO,
		}
	}

	#[must_use]
	pub(super) fn get_pos(&self) -> &Position {
		&self.pos
	}
}

impl<I> Iterator for PosStream<I> where I: Iterator<Item = CodePoint> {
	type Item = CodePoint;

	fn next(&mut self) -> Option<Self::Item> {
		let c = self.inner.next()?;
		if c == CodePoint::from_char('\n') {
			self.pos.line += 1;
			self.pos.column = 0;
		} else {
			self.pos.column += c.code_unit_count() as usize;
		}
		Some(c)
	}
}

#[cfg(test)]
mod tests {
	use maiscript_string::CodePoint;

	use super::*;

	fn from_utf16<I>(chars: impl IntoIterator<IntoIter = I>) -> PosStream<maiscript_string::DecodeUtf16<I>> 
		where I: Iterator<Item = u16>,
	{
		let decoder = CodePoint::decode_utf16(chars);
		PosStream::new(decoder)
	}

	#[test]
	fn surrogate_pair_is_retrieved_at_once() {
		let source = "\u{1F92F}".encode_utf16();
		let mut stream = from_utf16(source);
		assert_eq!(stream.next(), Some(CodePoint::from_char('\u{1F92F}')));
	}

	#[test]
	fn start_with_position_zero() {
		let source = "abc".encode_utf16();
		let stream = from_utf16(source);
		assert_eq!(stream.get_pos(), &Position::ZERO);
	}

	#[test]
	fn lf_is_counted() {
		let source = "a\nb".encode_utf16();
		let mut stream = from_utf16(source);
		stream.next();
		assert_eq!(stream.next(), Some(CodePoint::from_char('\n')));
		assert_eq!(stream.get_pos(), &Position { line: 1, column: 0 });
	}
}
