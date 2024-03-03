mod cell;

use yew::prelude::*;

use web_sys::HtmlTextAreaElement;

use cell::{CellKind, CellsResult};

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
	let (height, width) = cells.as_ref().map_or((0, 0), |cells| cells.get_size());

	html! {
		<div class="view">
		{
			(0..height * width)
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
			<table>
				<thead>
					<tr>
						<th scope="col">{"文字"}</th>
						<th scope="col">{"マス"}</th>
					</tr>
				</thead>
				<tbody>
					<tr>
						<th scope="row">
							<pre><code>{' '}</code></pre>
							<pre><code>{'.'}</code></pre>
						</th>
						<td>{"空きマス"}</td>
					</tr>
					<tr>
						<th scope="row">
							<pre><code>{'#'}</code></pre>
						</th>
						<td>{"壁"}</td>
					</tr>
					<tr>
						<th scope="row">
						<pre><code>{','}</code></pre>
						<pre><code>{'p'}</code></pre>
						</th>
						<td>{"穴・空域"}</td>
					</tr>
					<tr>
						<th scope="row">
							<pre><code>{'~'}</code></pre>
							<pre><code>{'w'}</code></pre>
						</th>
						<td>{"水"}</td>
					</tr>
					<tr>
						<th scope="row">
							<pre><code>{'b'}</code></pre>
						</th>
						<td>{"ボヨヨン壁"}</td>
					</tr>
				</tbody>
			</table>
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
	let cells = use_state(|| TryFrom::try_from(INITIAL_CELLS));

	html! {
		<div class="bouncy_walls">
			<View cells={cells.clone()} />
			<InputArea cells={cells} />
		</div>
	}
}