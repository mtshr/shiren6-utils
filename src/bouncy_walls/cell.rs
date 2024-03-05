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

	pub fn is_solid(&self) -> bool {
		use CellKind::*;

		matches!(*self, Wall | BouncyWall)
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
	// 1 [#][#] 2 [#][/] 4 [\][/] 8 [\][#]
	//   [/][\]   [#][\]   [#][#]   [/][#]
	const BOUNCE_TOP: u32 = 1;
	const BOUNCE_LEFT: u32 = 2;
	const BOUNCE_BOTTOM: u32 = 4;
	const BOUNCE_RIGHT: u32 = 8;

	pub fn get(&self, y: usize, x: usize) -> Option<CellKind> {
		(y < self.height && x < self.width).then(|| self.kinds[y * self.width + x])
	}

	pub fn get_size(&self) -> (usize, usize) {
		(self.height, self.width)
	}

	pub fn len(&self) -> usize {
		self.kinds.len()
	}

	pub fn find_routes(&self) -> Vec<usize> {
		let mut representatives = Vec::new();

		let mut visited = vec![false; self.len() * 2];

		for (i, _) in self
			.kinds
			.iter()
			.enumerate()
			.filter(|(_, kind)| !kind.is_solid())
		{
			for v in [i, i + self.len()].into_iter() {
				if visited[v] {
					continue;
				}

				let bounce_flag = self.dfs(v, &mut visited, |_| {});

				if bounce_flag
					== Self::BOUNCE_TOP
						| Self::BOUNCE_LEFT | Self::BOUNCE_BOTTOM
						| Self::BOUNCE_RIGHT
				{
					representatives.push(v);
				}
			}
		}

		representatives
	}

	pub fn trace(&self, v: usize) -> Vec<u8> {
		let mut trace = vec![0; self.len()];

		let mut visited = vec![false; self.len() * 2];

		self.dfs(v, &mut visited, |v| {
			let layer = v / self.len();
			trace[v % self.len()] |= 1 << layer;
		});

		trace
	}

	fn dfs<F>(&self, v: usize, visited: &mut [bool], mut f: F) -> u32
	where
		F: FnMut(usize),
	{
		let mut bounce_flag = 0;

		let mut stack = vec![v];
		visited[v] = true;

		let decode = |v: usize| (v / self.len(), v % self.len() / self.width, v % self.width);
		let encode = |layer: usize, y: usize, x: usize| layer * self.len() + y * self.width + x;

		while let Some(v) = stack.pop() {
			f(v);

			let (layer, y, x) = decode(v);
			let add_delta = |dy: isize, dx: isize| {
				y.checked_add_signed(dy)
					.filter(|&y| y < self.height)
					.zip(x.checked_add_signed(dx).filter(|&x| x < self.width))
			};
			let dy = if layer == 0 { [-1, 1] } else { [1, -1] };

			for (dy, dx) in dy.into_iter().zip([-1, 1]) {
				let Some((ny, nx)) = add_delta(dy, dx) else {
					continue;
				};
				let nkind = self.get(ny, nx).unwrap();
				if !nkind.is_solid() {
					// go straight
					let nv = encode(layer, ny, nx);
					if visited[nv] {
						continue;
					}
					visited[nv] = true;
					stack.push(nv);
				} else {
					use CellKind::BouncyWall;
					let adj_y = add_delta(dy, 0).unwrap();
					let adj_x = add_delta(0, dx).unwrap();
					match (
						self.get(adj_y.0, adj_y.1).unwrap(),
						self.get(adj_x.0, adj_x.1).unwrap(),
					) {
						(adj_y, adj_x) if adj_y.is_solid() && !adj_x.is_solid() => {
							if nkind == BouncyWall && adj_y == BouncyWall {
								bounce_flag |= if dy == -1 {
									Self::BOUNCE_TOP
								} else {
									Self::BOUNCE_BOTTOM
								};
							}
							let nv = encode(layer ^ 1, y, nx);
							if visited[nv] {
								continue;
							}
							visited[nv] = true;
							stack.push(nv);
						}
						(adj_y, adj_x) if !adj_y.is_solid() && adj_x.is_solid() => {
							if nkind == BouncyWall && adj_x == BouncyWall {
								bounce_flag |= if dx == -1 {
									Self::BOUNCE_LEFT
								} else {
									Self::BOUNCE_RIGHT
								};
							}
							let nv = encode(layer ^ 1, ny, x);
							if visited[nv] {
								continue;
							}
							visited[nv] = true;
							stack.push(nv);
						}
						_ => {}
					}
				}
			}
		}

		bounce_flag
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
