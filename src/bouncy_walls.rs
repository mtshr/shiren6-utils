use std::iter;

use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CellKind {
	Vacant,
	Wall,
	Pit,
	Water,
	BouncyWall,
}

impl CellKind {
	fn get_bg_class_name(&self) -> &'static str {
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
struct Cells {
	kinds: Vec<CellKind>,
	width: usize,
	height: usize,
}

impl Cells {
	fn get(&self, y: usize, x: usize) -> Option<CellKind> {
		(y < self.height && x < self.width).then(|| self.kinds[y * self.width + x])
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellsError {
	InvalidChar,
	TooLarge,
}

impl TryFrom<&str> for Cells {
	type Error = CellsError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let (height, width) = value
			.lines()
			.enumerate()
			.map(|(i, line)| (i, line.chars().count()))
			.fold((0, 0), |r, (i, c)| (i + 1, r.1.max(c)));

		if height > 16 || width > 16 {
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

#[derive(PartialEq, Properties)]
pub struct CellProperties {
	cells: UseStateHandle<Result<Cells, CellsError>>,
	y: usize,
	x: usize,
}

#[function_component(Cell)]
fn cell_panel(props: &CellProperties) -> Html {
	let CellProperties { x, y, .. } = props;
	let cell = props
		.cells
		.as_ref()
		.unwrap()
		.get(*y, *x)
		.expect("Could not obtain the cell kind.");

	html! {
		<div class={classes!("cell", cell.get_bg_class_name())} style={format!("grid-row: {}; grid-column: {};", props.y + 1, props.x + 1)}>
		if cell == CellKind::BouncyWall {
			<div class="cell_inner">
			</div>
		}
		</div>
	}
}

#[function_component(View)]
fn view() -> Html {
	let cells = use_state(|| {
		<Cells as TryFrom<_>>::try_from(
			r#"#bbbbbbbb##
bb......bbb
b
b.........b
b.........b
bbbb....bbb
###bbb bb##"#,
		)
	});

	let (len, width) = cells
		.as_ref()
		.map_or((0, 0), |cells| (cells.kinds.len(), cells.width));

	html! {
		<div class="view">
		{
			(0..len)
			.map(|i| {
				html! {
					<Cell cells={cells.clone()} y={i / width} x={i % width} />
				}
			})
			.collect::<Html>()
		}
		</div>
	}
}

#[function_component(BouncyWalls)]
pub fn bouncy_walls() -> Html {
	html! {
		<div class="bouncy_walls">
			<View />
		</div>
	}
}
