use syntax::{EsString, IdentifierName, Range};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenContent {
	EOF,
	NewLine,

	/// Identifier or Keyword
	IdentifierName(IdentifierName),

	// literal
	NumberLiteral(EsString),
	StringLiteral(EsString),

	// template string
	TemplateStart,

	/// "!"
	Not,
	/// "!="
	NotEq,
	/// "#"
	Sharp,
	/// "#["
	OpenSharpBracket,
	/// "###"
	Sharp3,
	/// "%"
	Percent,
	/// "&&"
	And2,
	/// "("
	OpenParen,
	/// ")"
	CloseParen,
	/// "*"
	Asterisk,
	/// "+"
	Plus,
	/// "+="
	PlusEq,
	/// ","
	Comma,
	/// "-"
	Minus,
	/// "-="
	MinusEq,
	/// "."
	Dot,
	/// "/"
	Slash,
	/// ":"
	Colon,
	/// "::"
	Colon2,
	/// ";"
	SemiColon,
	/// "<"
	Lt,
	/// "<="
	LtEq,
	/// "<:"
	Out,
	/// "="
	Eq,
	/// "=="
	Eq2,
	/// "=>"
	Arrow,
	/// ">"
	Gt,
	/// ">="
	GtEq,
	/// "?"
	Question,
	/// "@"
	At,
	/// "["
	OpenBracket,
	/// "\\"
	BackSlash,
	/// "]"
	CloseBracket,
	/// "^"
	Hat,
	/// "{"
	OpenBrace,
	/// "|"
	Or,
	/// "||"
	Or2,
	/// "}"
	CloseBrace,
}

#[derive(Debug)]
pub(crate) struct Token {
	pub(crate) content: TokenContent,
	pub(crate) range: Range,
	pub(crate) has_left_spacing: bool,
}

#[derive(Debug)]
pub(crate) enum TemplateTokenContent {
	/// string until "{", where last "{" is omitted
	Part(EsString),
	/// "`"
	End,
}

#[derive(Debug)]
pub(crate) struct TemplateToken {
	pub(crate) content: TemplateTokenContent,
	pub(crate) range: Range,
}
