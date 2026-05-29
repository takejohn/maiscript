use maiscript_string::CodePoint;

use crate::{
    error::{AiScriptSyntaxError, AiScriptSyntaxErrorSource, Result},
    lexical::{char_stream::PeekableStream, code_points::*},
};

pub(super) trait SkipSpace {
    /// Skips spacing CodePoints.
    /// Comments and new lines are not skipped.
    /// Returns true if has skipped spacing and false otherwise.
    fn skip_space(&mut self) -> bool;

    /// Skips a single comment if the cursor is on the comment start.
    /// Returns Ok(true) if has skipped a comment and false otherwise.
    /// Returns an error if a block comment without a closing character sequence found.
    fn skip_comment(&mut self) -> Result<bool>;

    /// Skips whitespace and comments.
    /// Whitespace includes spaces and new lines.
    /// Returns an error if a block comment without a closing character sequence found.
    fn skip_whitespace_and_comments(&mut self) -> Result<()>;
}

impl<I> SkipSpace for PeekableStream<I>
where
    I: Iterator<Item = CodePoint>,
{
    /// Returns true if has skipped spacing and false otherwise.
    fn skip_space(&mut self) -> bool {
        if self.peek(0).is_some_and(is_space_char) {
            self.next();
            skip(self, is_space_char);
            true
        } else {
            false
        }
    }

    fn skip_comment(&mut self) -> Result<bool> {
        if self.peek(0).is_none_or(|c| c != CodePoint::from_char('/')) {
            return Ok(false);
        }
        let Some(next) = self.peek(1) else {
            return Ok(false);
        };
        if next == CodePoint::from_char('*') {
            skip_block_comment(self).map(|_| true)
        } else if next == CodePoint::from_char('/') {
            skip_line_comment(self);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<()> {
        loop {
            skip(self, is_whitespace_char);
            if !self.skip_comment()? {
                break;
            }
        }
        Ok(())
    }
}

fn skip_line_comment(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) {
    stream.next();
    stream.next();
    skip(stream, |c| c != CodePoint::from_char('\n'));
}

fn skip_block_comment(stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>) -> Result<()> {
    stream.next();
    stream.next();
    loop {
        if require_next_char(stream)? != CodePoint::from_char('*') {
            continue;
        }
        if require_next_char(stream)? != CodePoint::from_char('/') {
            continue;
        }
        break Ok(());
    }
}

fn require_next_char(
    stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>,
) -> Result<CodePoint> {
    let c = stream.next();
    c.ok_or_else(|| AiScriptSyntaxError {
        source: AiScriptSyntaxErrorSource::UnexpectedEOF,
        pos: stream.get_pos().clone(),
    })
}

fn skip(
    stream: &mut PeekableStream<impl Iterator<Item = CodePoint>>,
    mut predicate: impl FnMut(CodePoint) -> bool,
) {
    while stream.peek(0).is_some_and(&mut predicate) {
        stream.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod skip_spacing {
        use super::*;

        #[test]
        fn empty() {
            let source = Vec::<CodePoint>::new();
            let mut stream = PeekableStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, false);
            assert_eq!(stream.peek(0), None);
        }

        #[test]
        fn only_space() {
            let source = vec![CodePoint::from_char(' ')];
            let mut stream = PeekableStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, true);
            assert_eq!(stream.peek(0), None);
        }

        #[test]
        fn no_leading_spaces() {
            let soruce = vec![CodePoint::from_char('a')];
            let mut stream = PeekableStream::new(soruce);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, false);
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('a')));
        }

        #[test]
        fn leading_spaces() {
            let soruce = vec![
                CodePoint::from_char(' '),
                CodePoint::from_char(' '),
                CodePoint::from_char('a'),
            ];
            let mut stream = PeekableStream::new(soruce);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, true);
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('a')));
        }
    }

    mod skip_whitespace_and_comments {
        use maiscript_syntax::Position;

        use super::*;

        #[test]
        fn new_lines() {
            let source = vec![
                CodePoint::from_char('\n'),
                CodePoint::from_char('\n'),
                CodePoint::from_char('a'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            result.expect("should have been ok");
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('a')));
        }

        #[test]
        fn line_comment() {
            let source = vec![
                CodePoint::from_char('/'),
                CodePoint::from_char('/'),
                CodePoint::from_char('a'),
                CodePoint::from_char('\n'),
                CodePoint::from_char('b'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            result.expect("should have been ok");
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('b')));
        }

        #[test]
        fn closed_block_comment() {
            let source = vec![
                CodePoint::from_char('/'),
                CodePoint::from_char('*'),
                CodePoint::from_char('a'),
                CodePoint::from_char('*'),
                CodePoint::from_char('/'),
                CodePoint::from_char('b'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_ok());
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('b')));
        }

        #[test]
        fn error_with_opened_block_comment() {
            let source = vec![
                CodePoint::from_char('/'),
                CodePoint::from_char('*'),
                CodePoint::from_char('a'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(
                result.is_err_and(|e| matches!(e.source, AiScriptSyntaxErrorSource::UnexpectedEOF))
            );
        }

        #[test]
        fn error_with_opened_block_comment_asterisk() {
            let source = vec![
                CodePoint::from_char('/'),
                CodePoint::from_char('*'),
                CodePoint::from_char('a'),
                CodePoint::from_char('*'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_err_and(|e| {
                matches!(e.source, AiScriptSyntaxErrorSource::UnexpectedEOF)
                    && e.pos == Position { line: 0, column: 4 }
            }));
        }

        #[test]
        fn multiple_comments() {
            let source = vec![
                CodePoint::from_char('/'),
                CodePoint::from_char('*'),
                CodePoint::from_char('a'),
                CodePoint::from_char('*'),
                CodePoint::from_char('/'),
                CodePoint::from_char('\n'),
                CodePoint::from_char('/'),
                CodePoint::from_char('/'),
                CodePoint::from_char('b'),
                CodePoint::from_char('\n'),
                CodePoint::from_char('c'),
            ];
            let mut stream = PeekableStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_ok());
            assert_eq!(stream.peek(0), Some(CodePoint::from_char('c')));
        }
    }
}
