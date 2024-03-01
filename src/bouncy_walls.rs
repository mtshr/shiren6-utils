use std::iter;

use yew::prelude::*;

use web_sys::HtmlTextAreaElement;

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

type CellsResult = Result<Cells, CellsError>;

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

#[derive(PartialEq, Properties)]
struct CellProperties {
	cells: UseStateHandle<CellsResult>,
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

#[derive(PartialEq, Properties)]
struct ViewProperties {
	cells: UseStateHandle<CellsResult>,
}

#[function_component(View)]
fn view(props: &ViewProperties) -> Html {
	let cells = props.cells.clone();
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

#[derive(PartialEq, Properties)]
struct InputAreaProperties {
	cells: UseStateHandle<CellsResult>,
}

#[function_component(InputArea)]
fn input_area(props: &InputAreaProperties) -> Html {
	let node_ref = use_node_ref();

	let value_handle = use_state(|| AttrValue::from(INITIAL_CELLS));
	let value = (*value_handle).clone();

	let oninput = {
		let node_ref = node_ref.clone();
		let cells_handle = props.cells.clone();

		Callback::from(move |_| {
			let textarea = node_ref.cast::<HtmlTextAreaElement>();

			if let Some(input) = textarea {
				let value = input.value();
				let cells: CellsResult = TryFrom::try_from(value.as_ref());
				cells_handle.set(cells);
				value_handle.set(AttrValue::from(value));
			}
		})
	};

	let textarea_class = props.cells.as_ref().map_or("error", |_| "");

	html! {
		<div class="input_area">
			<textarea class={textarea_class} ref={node_ref} rows={10} columns={32} oninput={oninput} spellcheck="false" value={value}/>
		</div>
	}
}

const INITIAL_CELLS: &str = r#"#bbbbbbbb##
bb......bbb
b
b.........b
b.........b
bbbb....bbb
###bbb.bb##
"#;

#[function_component(BouncyWalls)]
pub fn bouncy_walls() -> Html {
	let cells = use_state(|| <Cells as TryFrom<_>>::try_from(INITIAL_CELLS));

	html! {
		<div class="bouncy_walls">
			<View cells={cells.clone()} />
			<InputArea cells={cells} />
		</div>
	}
}
