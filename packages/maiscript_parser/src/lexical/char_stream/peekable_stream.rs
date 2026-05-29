use std::collections::VecDeque;

use maiscript_string::CodePoint;
use maiscript_syntax::Position;

use crate::lexical::char_stream::pos_stream::PosStream;

#[derive(Debug)]
struct BufferEntry {
    code_point: CodePoint,
    pos: Position,
}

#[derive(Debug)]
pub(crate) struct PeekableStream<I>
where
    I: Iterator<Item = CodePoint>,
{
    inner: PosStream<I>,
    pos: Position,
    buffer: VecDeque<BufferEntry>,
}

impl<I> PeekableStream<I>
where
    I: Iterator<Item = CodePoint>,
{
    pub(crate) fn new(inner: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            inner: PosStream::new(inner.into_iter()),
            pos: Position::ZERO,
            buffer: VecDeque::new(),
        }
    }

    #[must_use]
    pub(crate) fn get_pos(&self) -> &Position {
        &self.pos
    }

    #[must_use]
    pub(crate) fn peek(&mut self, index: usize) -> Option<CodePoint> {
        match self.buffer.get(index) {
            Some(entry) => Some(entry.code_point),
            None => {
                while let Some(code_point) = self.inner.next() {
                    let pos = self.inner.get_pos().clone();
                    self.buffer.push_back(BufferEntry { code_point, pos });
                    if self.buffer.len() == index + 1 {
                        return Some(code_point);
                    }
                }
                None
            }
        }
    }

    pub(crate) fn next_if(
        &mut self,
        predicate: impl FnOnce(CodePoint) -> bool,
    ) -> Option<CodePoint> {
        let peeked = self.peek(0).filter(|&cp| predicate(cp));
        if peeked.is_some() {
            self.next();
        }
        peeked
    }

    pub(crate) fn next_if_eq(&mut self, expected: CodePoint) -> Option<CodePoint> {
        self.next_if(|cp| cp == expected)
    }
}

impl<I> Iterator for PeekableStream<I>
where
    I: Iterator<Item = CodePoint>,
{
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.pop_front() {
            Some(BufferEntry { code_point, pos }) => {
                self.pos = pos;
                Some(code_point)
            }
            None => {
                let maybe_code_point = self.inner.next();
                self.pos = self.inner.get_pos().clone();
                maybe_code_point
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: [CodePoint; 3] = [
        CodePoint::from_char('a'),
        CodePoint::from_char('b'),
        CodePoint::from_char('c'),
    ];

    #[test]
    fn starts_with_position_zero() {
        let stream = PeekableStream::new(SOURCE);
        assert_eq!(*stream.get_pos(), Position::ZERO);
    }

    #[test]
    fn position_is_not_changed_by_peek() {
        let mut stream = PeekableStream::new(SOURCE);
        let peeked = stream.peek(0);
        assert_eq!(peeked, Some(CodePoint::from_char('a')));
        assert_eq!(*stream.get_pos(), Position::ZERO);
    }

    #[test]
    fn position_is_changed_by_next() {
        let mut stream = PeekableStream::new(SOURCE);
        let next = stream.next();
        assert_eq!(next, Some(CodePoint::from_char('a')));
        assert_eq!(*stream.get_pos(), Position { line: 0, column: 1 });
    }

    #[test]
    fn get_peeked_value_by_next() {
        let mut stream = PeekableStream::new(SOURCE);
        let peeked = stream.peek(0);
        assert_eq!(peeked, Some(CodePoint::from_char('a')));
        let next = stream.next();
        assert_eq!(next, Some(CodePoint::from_char('a')));
    }

    #[test]
    fn peek_is_idempotent() {
        let mut stream = PeekableStream::new(SOURCE);
        let peeked_first = stream.peek(0);
        assert_eq!(peeked_first, Some(CodePoint::from_char('a')));
        let peeked_second = stream.peek(0);
        assert_eq!(peeked_second, Some(CodePoint::from_char('a')));
    }

    #[test]
    fn peek_further() {
        let mut stream = PeekableStream::new(SOURCE);
        let peeked = stream.peek(1);
        assert_eq!(peeked, Some(CodePoint::from_char('b')));
    }
}
