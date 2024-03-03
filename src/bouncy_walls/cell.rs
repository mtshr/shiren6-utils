use std::iter;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellKind {
	Vacant,
	Wall,
	Pit,
	Water,
	BouncyWall,
}

impl CellKind {
	pub fn get_bg_class_name(&self) -> &'static str {
		use CellKind::*;

		match self {
			Vacant => "vacant_bg",
			Wall => "wall_bg",
			Pit => "pit_bg",
			Water => "water_bg",
			BouncyWall => "bouncy_wall_bg",
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Cells {
	kinds: Vec<CellKind>,
	width: usize,
	height: usize,
}

pub type CellsResult = Result<Cells, CellsError>;

impl Cells {
	pub fn get(&self, y: usize, x: usize) -> Option<CellKind> {
		(y < self.height && x < self.width).then(|| self.kinds[y * self.width + x])
	}

	pub fn get_size(&self) -> (usize, usize) {
		(self.height, self.width)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellsError {
	InvalidChar,
	TooLarge,
}

impl TryFrom<&str> for Cells {
	type Error = CellsError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let value = value.trim_end().trim_start_matches("\n");
		let (height, width) = value
			.lines()
			.enumerate()
			.map(|(i, line)| (i, line.chars().count()))
			.fold((0, 0), |r, (i, c)| (i + 1, r.1.max(c)));

		if height > 24 || width > 24 {
			return Err(CellsError::TooLarge);
		}

		let mut kinds = Vec::with_capacity(height * width);

		for line in value.lines() {
			for c in line.chars().chain(iter::repeat_with(|| '.')).take(width) {
				use CellKind::*;
				let kind = match c {
					' ' | '.' => Vacant,
					'#' => Wall,
					',' | 'p' | 'P' => Pit,
					'~' | 'w' | 'W' => Water,
					'b' | 'B' => BouncyWall,
					_ => return Err(CellsError::InvalidChar),
				};
				kinds.push(kind);
			}
		}

		Ok(Self {
			kinds,
			width,
			height,
		})
	}
}
