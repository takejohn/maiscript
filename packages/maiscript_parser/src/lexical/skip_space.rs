use boa_string::CodePoint;
use maiscript_char_stream::CharStream;

use crate::{
    error::{AiScriptSyntaxError, AiScriptSyntaxErrorSource, Result},
    lexical::code_points::*,
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

impl SkipSpace for CharStream<'_> {
    /// Returns true if has skipped spacing and false otherwise.
    fn skip_space(&mut self) -> bool {
        if self.char().is_some_and(is_space_char) {
            self.next();
            skip(self, is_space_char);
            true
        } else {
            false
        }
    }

    fn skip_comment(&mut self) -> Result<bool> {
        if self.char().is_none_or(|c| c != CodePoint::Unicode('/')) {
            return Ok(false);
        }
        let Some(next) = self.char_nth(1) else {
            return Ok(false);
        };
        if next == CodePoint::Unicode('*') {
            skip_block_comment(self).map(|_| true)
        } else if next == CodePoint::Unicode('/') {
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

fn skip_line_comment(stream: &mut CharStream<'_>) {
    stream.next();
    stream.next();
    skip(stream, |c| c != CodePoint::Unicode('\n'));
}

fn skip_block_comment(stream: &mut CharStream<'_>) -> Result<()> {
    stream.next();
    stream.next();
    loop {
        if require_next_char(stream)? != CodePoint::Unicode('*') {
            continue;
        }
        if require_next_char(stream)? != CodePoint::Unicode('/') {
            continue;
        }
        break Ok(());
    }
}

fn require_next_char(stream: &mut CharStream<'_>) -> Result<CodePoint> {
    let c = stream.next();
    c.ok_or_else(|| AiScriptSyntaxError {
        source: AiScriptSyntaxErrorSource::UnexpectedEOF,
        pos: stream.pos().clone(),
    })
}

fn skip(stream: &mut CharStream<'_>, mut predicate: impl FnMut(CodePoint) -> bool) {
    while stream.char().is_some_and(&mut predicate) {
        stream.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod skip_spacing {
        use boa_string::JsStr;
        use boa_string_literal::js_str;

        use super::*;

        #[test]
        fn empty() {
            let source = JsStr::EMPTY;
            let mut stream = CharStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, false);
            assert_eq!(stream.char(), None);
        }

        #[test]
        fn only_space() {
            let source = js_str!(" ");
            let mut stream = CharStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, true);
            assert_eq!(stream.char(), None);
        }

        #[test]
        fn no_leading_spaces() {
            let source = js_str!("a");
            let mut stream = CharStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, false);
            assert_eq!(stream.char(), Some(CodePoint::Unicode('a')));
        }

        #[test]
        fn leading_spaces() {
            let source = js_str!("  a");
            let mut stream = CharStream::new(source);
            let has_leading_space = stream.skip_space();
            assert_eq!(has_leading_space, true);
            assert_eq!(stream.char(), Some(CodePoint::Unicode('a')));
        }
    }

    mod skip_whitespace_and_comments {
        use boa_string_literal::js_str;
        use maiscript_syntax::Position;

        use super::*;

        #[test]
        fn new_lines() {
            let source = js_str!("\n\na");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            result.expect("should have been ok");
            assert_eq!(stream.char(), Some(CodePoint::Unicode('a')));
        }

        #[test]
        fn line_comment() {
            let source = js_str!("//a\nb");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            result.expect("should have been ok");
            assert_eq!(stream.char(), Some(CodePoint::Unicode('b')));
        }

        #[test]
        fn closed_block_comment() {
            let source = js_str!("/*a*/b");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_ok());
            assert_eq!(stream.char(), Some(CodePoint::Unicode('b')));
        }

        #[test]
        fn error_with_opened_block_comment() {
            let source = js_str!("/*a");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(
                result.is_err_and(|e| matches!(e.source, AiScriptSyntaxErrorSource::UnexpectedEOF))
            );
        }

        #[test]
        fn error_with_opened_block_comment_asterisk() {
            let source = js_str!("/*a*");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_err_and(|e| {
                matches!(e.source, AiScriptSyntaxErrorSource::UnexpectedEOF)
                    && e.pos == Position { line: 0, column: 4 }
            }));
        }

        #[test]
        fn multiple_comments() {
            let source = js_str!("/*a*/\n//b\nc");
            let mut stream = CharStream::new(source);
            let result = stream.skip_whitespace_and_comments();
            assert!(result.is_ok());
            assert_eq!(stream.char(), Some(CodePoint::Unicode('c')));
        }
    }
}
