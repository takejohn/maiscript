use maiscript_string::{CodePoint, EsStr};
use maiscript_syntax::Position;

/// End-of-line sequences defined in Language Server Protocol.
const EOL: &[&EsStr] = &[
    EsStr::from_utf16(&['\n' as u16]),
    EsStr::from_utf16(&['\r' as u16, '\n' as u16]),
    EsStr::from_utf16(&['\r' as u16]),
];

#[derive(Debug, Clone)]
pub struct CharStream<'a> {
    source: &'a EsStr,
    line_start: usize,
    line_index: usize,
    column_index: usize,
}

impl<'a> CharStream<'a> {
    pub fn new(source: &'a EsStr) -> Self {
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
        self.source.code_point_at(self.abs_index())
    }

    pub fn try_read_eol(&mut self) -> bool {
        let slice = &self.source[self.abs_index()..];
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
            return Some(CodePoint::from_char('\n'));
        }
        let cp = self.char()?;
        self.column_index += cp.code_unit_count() as usize;
        Some(cp)
    }
}

#[cfg(test)]
mod tests {
    use maiscript_string::EsString;

    use super::*;

    #[test]
    fn starts_with_position_zero() {
        let source = EsString::from("abc");
        let stream = CharStream::new(&source);
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_not_changed_by_char() {
        let source = EsString::from("abc");
        let stream = CharStream::new(&source);
        let cp = stream.char();
        assert_eq!(cp, Some(CodePoint::from_char('a')));
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_not_changed_by_char_nth() {
        let source = EsString::from("abc");
        let stream = CharStream::new(&source);
        let cp = stream.char_nth(1);
        assert_eq!(cp, Some(CodePoint::from_char('b')));
        assert_eq!(stream.pos(), Position::ZERO);
    }

    #[test]
    fn position_is_changed_by_next() {
        let source = EsString::from("abc");
        let mut stream = CharStream::new(&source);
        let cp = stream.next();
        assert_eq!(cp, Some(CodePoint::from_char('a')));
        assert_eq!(stream.pos(), Position { line: 0, column: 1 });
    }

    #[test]
    fn get_char_by_next() {
        let source = EsString::from("abc");
        let mut stream = CharStream::new(&source);
        let cp = stream.char();
        assert_eq!(cp, Some(CodePoint::from_char('a')));
        let next = stream.next();
        assert_eq!(next, Some(CodePoint::from_char('a')));
    }

    #[test]
    fn char_is_idempotent() {
        let source = EsString::from("abc");
        let stream = CharStream::new(&source);
        let cp1 = stream.char();
        assert_eq!(cp1, Some(CodePoint::from_char('a')));
        let cp2 = stream.char();
        assert_eq!(cp2, Some(CodePoint::from_char('a')));
    }

    #[test]
    fn test_char_nth() {
        let source = EsString::from("abc");
        let stream = CharStream::new(&source);
        let cp = stream.char_nth(1);
        assert_eq!(cp, Some(CodePoint::from_char('b')));
    }

    #[test]
    fn surrogate_pair_is_retrieved_at_once() {
        let source = EsString::from("\u{1F92F}");
        let mut stream = CharStream::new(&source);
        assert_eq!(stream.next(), Some(CodePoint::from_char('\u{1F92F}')));
        assert_eq!(stream.pos(), Position { line: 0, column: 2 });
    }

    #[test]
    fn lf_as_eol() {
        let source = EsString::from("a\nb");
        let mut stream = CharStream::new(&source);
        assert_eq!(stream.next(), Some(CodePoint::from_char('a')));
        assert_eq!(stream.next(), Some(CodePoint::from_char('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::from_char('b')));
    }

    #[test]
    fn crlf_as_eol() {
        let source = EsString::from("a\r\nb");
        let mut stream = CharStream::new(&source);
        assert_eq!(stream.next(), Some(CodePoint::from_char('a')));
        assert_eq!(stream.next(), Some(CodePoint::from_char('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::from_char('b')));
    }

    #[test]
    fn cr_as_eol() {
        let source = EsString::from("a\rb");
        let mut stream = CharStream::new(&source);
        assert_eq!(stream.next(), Some(CodePoint::from_char('a')));
        assert_eq!(stream.next(), Some(CodePoint::from_char('\n')));
        assert_eq!(stream.pos(), Position { line: 1, column: 0 });
        assert_eq!(stream.next(), Some(CodePoint::from_char('b')));
    }
}
