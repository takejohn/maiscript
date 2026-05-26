use maiscript_string::CodePoint;
use maiscript_syntax::Range;

use crate::{error::Result, lexical::{char_stream::PeekableStream, match_token::match_token, skip_space::SkipSpace, token::{TemplateToken, Token, TokenContent}}};

pub(crate) struct Lexer<I> where I: Iterator<Item = CodePoint> {
	stream: PeekableStream<I>,
}

impl<I> Lexer<I> where I: Iterator<Item = CodePoint> {
	pub(super) fn new(stream: PeekableStream<I>) -> Self {
		Self { stream }
	}

	pub(crate) fn read_default_mode(&mut self) -> Result<Token> {
		let has_left_spacing = self.stream.skip_space();
		self.stream.skip_comment()?;
		let start = self.stream.get_pos().clone();
		let content = match_token(&mut self.stream);
		let end = self.stream.get_pos().clone();
		let range = Range::new(start, end);
		if matches!(content, TokenContent::NewLine) {
			self.stream.skip_whitespace_and_comments()?;
		}
		Ok(Token { content, range, has_left_spacing })
	}

	pub(crate) fn read_template_mode(&mut self) -> TemplateToken {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_line_token_never_read_consecutively() {
		let source = "\n\n".chars().map(CodePoint::from_char);
		let mut stream = PeekableStream::new(source);
		let mut lexer = Lexer::new(stream);
		assert_eq!(lexer.read_default_mode().unwrap().content, TokenContent::NewLine);
		assert_eq!(lexer.read_default_mode().unwrap().content, TokenContent::EOF);
	}
}
