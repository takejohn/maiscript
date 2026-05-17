use std::fmt::Display;

/// line and column are zero-based indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
	/// zero-based index
	pub line: usize,
	/// zero-based index
	pub column: usize,
}

impl Position {
	pub const ZERO: Self = Self {
		line: 0,
		column: 0,
	};
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(Line {}, Column {})", self.line + 1, self.column + 1)
	}
}

impl PartialOrd for Position {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Position {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.line.cmp(&other.line).then_with(|| self.column.cmp(&other.column))
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
	/// inclusive
	pub start: Position,
	/// exclusive
	pub end: Position,
}

impl Range {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	pub fn zero_width(pos: Position) -> Self {
		let cloned = pos.clone();
		Self { start: pos, end: cloned }
	}
}
