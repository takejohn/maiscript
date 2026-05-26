use maiscript_string::CodePoint;

const SPACE_CHARS: [CodePoint; 2] = [CodePoint::from_char(' '), CodePoint::from_char('\t')];
const LINE_BREAK_CHARS: [CodePoint; 2] = [CodePoint::from_char('\r'), CodePoint::from_char('\n')];
const DIGIT_ZERO_U32: u32 = '0' as u32;
const DIGIT_NINE_U32: u32 = '9' as u32;
const UPPER_A_U32: u32 = 'A' as u32;
const UPPER_F_U32: u32 = 'F' as u32;
const UPPER_Z_U32: u32 = 'Z' as u32;
const LOWER_A_U32: u32 = 'a' as u32;
const LOWER_F_U32: u32 = 'f' as u32;
const LOWER_Z_U32: u32 = 'z' as u32;
const UNDERSCORE_U32: u32 = '_' as u32;

pub(super) const LINE_FEED: CodePoint = CodePoint::from_char('\n');
pub(super) const EXCLAMATION_MARK: CodePoint = CodePoint::from_char('!');
pub(super) const QUOTATION_MARK: CodePoint = CodePoint::from_char('"');
pub(super) const NUMBER_SIGN: CodePoint = CodePoint::from_char('#');
pub(super) const DOLLAR_SING: CodePoint = CodePoint::from_char('$');
pub(super) const PERCENT_SIGN: CodePoint = CodePoint::from_char('%');
pub(super) const AMPERSAND: CodePoint = CodePoint::from_char('&');
pub(super) const APOSTROPHE: CodePoint = CodePoint::from_char('\'');
pub(super) const LEFT_PARENTHESIS: CodePoint = CodePoint::from_char('(');
pub(super) const RIGHT_PARENTHESIS: CodePoint = CodePoint::from_char(')');
pub(super) const ASTERISK: CodePoint = CodePoint::from_char('*');
pub(super) const PLUS_SIGN: CodePoint = CodePoint::from_char('+');
pub(super) const COMMA: CodePoint = CodePoint::from_char(',');
pub(super) const HYPHEN_MINUS: CodePoint = CodePoint::from_char('-');
pub(super) const FULL_STOP: CodePoint = CodePoint::from_char('.');
pub(super) const SOLIDUS: CodePoint = CodePoint::from_char('/');
pub(super) const COLON: CodePoint = CodePoint::from_char(':');
pub(super) const SEMICOLON: CodePoint = CodePoint::from_char(';');
pub(super) const LESS_THAN_SIGN: CodePoint = CodePoint::from_char('<');
pub(super) const EQUALS_SIGN: CodePoint = CodePoint::from_char('=');
pub(super) const GREATER_THAN_SIGN: CodePoint = CodePoint::from_char('>');
pub(super) const QUESTION_MARK: CodePoint = CodePoint::from_char('?');
pub(super) const COMMERCIAL_AT: CodePoint = CodePoint::from_char('@');
pub(super) const LEFT_SQUARE_BRACKET: CodePoint = CodePoint::from_char('[');
pub(super) const REVERSE_SOLIDUS: CodePoint = CodePoint::from_char('\\');
pub(super) const RIGHT_SQUARE_BRACKET: CodePoint = CodePoint::from_char(']');
pub(super) const LEFT_CURLY_BRACKET: CodePoint = CodePoint::from_char('{');
pub(super) const VERTICAL_LINE: CodePoint = CodePoint::from_char('|');
pub(super) const RIGHT_CURLY_BRACKET: CodePoint = CodePoint::from_char('}');
pub(super) const CIRCUMFLEX_ACCENT: CodePoint = CodePoint::from_char('^');
pub(super) const GRAVE_ACCENT: CodePoint = CodePoint::from_char('`');

pub(super) fn is_space_char(c: CodePoint) -> bool {
	SPACE_CHARS.contains(&c)
}

pub(super) fn is_line_break_char(c: CodePoint) -> bool {
	LINE_BREAK_CHARS.contains(&c)
}

pub(super) fn is_whitespace_char(c: CodePoint) -> bool {
	is_space_char(c) || is_line_break_char(c)
}

pub(super) fn is_digit(c: CodePoint) -> bool {
	matches!(c.as_u32(), DIGIT_ZERO_U32..=DIGIT_NINE_U32)
}

pub(super) fn is_identifier_start(c: CodePoint) -> bool {
	matches!(c.as_u32(), UPPER_A_U32..=UPPER_Z_U32 | LOWER_A_U32..=LOWER_Z_U32 | UNDERSCORE_U32)
}

pub(super) fn is_identifier_part(c: CodePoint) -> bool {
	matches!(c.as_u32(), UPPER_A_U32..=UPPER_Z_U32 | LOWER_A_U32..=LOWER_Z_U32 | DIGIT_ZERO_U32..=DIGIT_NINE_U32 | UNDERSCORE_U32)
}

pub(super) fn is_hex_digit(c: CodePoint) -> bool {
	matches!(c.as_u32(), DIGIT_ZERO_U32..=DIGIT_NINE_U32 | UPPER_A_U32..=UPPER_F_U32 | LOWER_A_U32..=LOWER_F_U32)
}
