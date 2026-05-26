use maiscript_string::CodePoint;

use crate::lexical::char_stream::{peekable_stream::PeekableStream, pos_stream::PosStream};

/// Creates a PeekableStream 
pub(crate) fn create_char_stream<I>(chars: impl IntoIterator<IntoIter = I>) -> PeekableStream<impl Iterator<Item = CodePoint>>
	where I: Iterator<Item = u16>
{
	let decoder = CodePoint::decode_utf16(chars);
	// Iterator rejecting CodePoints of carriage return.
	let filter = decoder.filter((|c| *c != CodePoint::from_char('\r')) as fn (&CodePoint) -> bool);
	let pos_stream = PosStream::new(filter);
	let peekable_stream = PeekableStream::new(pos_stream);
	return peekable_stream;
}
