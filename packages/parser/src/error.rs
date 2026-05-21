use syntax::{CodePoint, EsString, Position};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Syntax error: {source} {pos}")]
pub struct AiScriptSyntaxError {
	pub source: AiScriptSyntaxErrorSource,
	pub pos: Position,
}

#[derive(Error, Debug)]
pub enum AiScriptSyntaxErrorSource {
	#[error("unexpected EOF")]
	UnexpectedEOF,
	#[error("invalid sequence of characters: \"{0}\"")]
	InvalidCharacterSequence(EsString),
	#[error("invalid character: \"{0}\"")]
	InvalidCharacter(CodePoint),
}

pub type Result<T> = std::result::Result<T, AiScriptSyntaxError>;
