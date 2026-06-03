use boa_string::{CodePoint, JsStr};
use boa_string_literal::js_str;
use maiscript_syntax::Position;

/// End-of-line sequences defined in Language Server Protocol.
const EOL: &[JsStr] = &[js_str!("\n"), js_str!("\r\n"), js_str!("\r")];

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
    source: JsStr<'a>,
    line_start: usize,
    line_index: usize,
    column_index: usize,
}

impl<'a> CharStream<'a> {
    pub fn new(source: JsStr<'a>) -> Self {
        Self {
            source,
            line_start: 0,
            line_index: 0,
            column_index: 0,
        }
    }

    fn abs_index(&self) -> usize {
        self.line_start + self.column_index
    }

    pub fn pos(&self) -> Position {
        Position {
            line: self.line_index,
            column: self.column_index,
        }
    }

    pub fn char(&self) -> Option<CodePoint> {
        let index = self.abs_index();
        let len = self.source.len();
        (index < len).then(|| self.source.code_point_at(index))
    }

    pub fn try_read_eol(&mut self) -> bool {
        let slice = self.source.get_expect(self.abs_index()..);
        for &eol in EOL {
            if slice.starts_with(eol) {
                self.line_start = self.abs_index() + eol.len();
                self.line_index += 1;
                self.column_index = 0;
                return true;
            }
        }
        false
    }

    pub fn char_nth(&self, offset: usize) -> Option<CodePoint> {
        let mut iter = self.clone();
        iter.nth(offset)
    }

    pub fn next_if(&mut self, f: impl FnOnce(CodePoint) -> bool) -> Option<CodePoint> {
        let next = self.char()?;
        if f(next) {
            self.next();
            Some(next)
        } else {
            None
        }
    }

    pub fn next_if_eq(&mut self, expected: CodePoint) -> Option<CodePoint> {
        self.next_if(|cp| cp == expected)
    }
}

impl<'a> Iterator for CharStream<'a> {
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.try_read_eol() {
            return Some(CodePoint::Unicode('\n'));
        }
        let cp = self.char()?;
        self.column_index += cp.code_unit_count() as usize;
        Some(cp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_position_zero() {
        let source = js_str!("abc");
        let stream = CharStream::new(source);
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_not_changed_by_char() {
        let source = js_str!("abc");
        let stream = CharStream::new(source);
        let cp = stream.char();
        assert_eq!(cp, Some(CodePoint::Unicode('a')));
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_not_changed_by_char_nth() {
        let source = js_str!("abc");
        let stream = CharStream::new(source);
        let cp = stream.char_nth(1);
        assert_eq!(cp, Some(CodePoint::Unicode('b')));
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_changed_by_next() {
        let source = js_str!("abc");
        let mut stream = CharStream::new(source);
        let cp = stream.next();
        assert_eq!(cp, Some(CodePoint::Unicode('a')));
        assert_eq!(stream.pos(), Position { line: 0, column: 1 });
    }

    #[test]
    fn get_char_by_next() {
        let source = js_str!("abc");
        let mut stream = CharStream::new(source);
        let cp = stream.char();
        assert_eq!(cp, Some(CodePoint::Unicode('a')));
        let next = stream.next();
        assert_eq!(next, Some(CodePoint::Unicode('a')));
    }

    #[test]
    fn char_is_idempotent() {
        let source = js_str!("abc");
        let stream = CharStream::new(source);
        let cp1 = stream.char();
        assert_eq!(cp1, Some(CodePoint::Unicode('a')));
        let cp2 = stream.char();
        assert_eq!(cp2, Some(CodePoint::Unicode('a')));
    }

    #[test]
    fn test_char_nth() {
        let source = js_str!("abc");
        let stream = CharStream::new(source);
        let cp = stream.char_nth(1);
        assert_eq!(cp, Some(CodePoint::Unicode('b')));
    }

    #[test]
    fn surrogate_pair_is_retrieved_at_once() {
        let source = js_str!("\u{1F92F}");
        let mut stream = CharStream::new(source);
        assert_eq!(stream.next(), Some(CodePoint::Unicode('\u{1F92F}')));
        assert_eq!(stream.pos(), Position { line: 0, column: 2 });
    }

    #[test]
    fn lf_as_eol() {
        let source = js_str!("a\nb");
        let mut stream = CharStream::new(source);
        assert_eq!(stream.next(), Some(CodePoint::Unicode('a')));
        assert_eq!(stream.next(), Some(CodePoint::Unicode('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::Unicode('b')));
    }

    #[test]
    fn crlf_as_eol() {
        let source = js_str!("a\r\nb");
        let mut stream = CharStream::new(source);
        assert_eq!(stream.next(), Some(CodePoint::Unicode('a')));
        assert_eq!(stream.next(), Some(CodePoint::Unicode('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::Unicode('b')));
    }

    #[test]
    fn cr_as_eol() {
        let source = js_str!("a\rb");
        let mut stream = CharStream::new(source);
        assert_eq!(stream.next(), Some(CodePoint::Unicode('a')));
        assert_eq!(stream.next(), Some(CodePoint::Unicode('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::Unicode('b')));
    }
}
