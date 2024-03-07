mod cell;

use yew::prelude::*;

use web_sys::{HtmlSelectElement, HtmlTextAreaElement};

use cell::{CellKind, CellsResult};

#[derive(PartialEq, Properties)]
struct CellProperties {
	cells: UseStateHandle<CellsResult>,
	y: usize,
	x: usize,
	path: UseStateHandle<Option<Vec<u8>>>,
}

#[function_component(Cell)]
fn cell_panel(props: &CellProperties) -> Html {
	let CellProperties { x, y, path, .. } = props;
	let cells = props.cells.as_ref().unwrap();
	let cell = cells.get(*y, *x).expect("Could not obtain the cell kind.");

	let (_, width) = cells.get_size();
	// mathematically `path == 3` should not happen
	let path = path.as_ref().map_or(0, |path| path[y * width + x] as usize);

	html! {
		<>
			<div class={classes!("cell", cell.get_bg_class_name())} style={format!("grid-row: {}; grid-column: {};", props.y + 1, props.x + 1)}>
			if cell == CellKind::BouncyWall {
				<div class="cell_inner">
				</div>
			}
			</div>
			<div class={classes!("path", ["", "diagonal1", "diagonal2"][path])} style={format!("grid-row: {}; grid-column: {};", props.y + 1, props.x + 1)}>
			</div>
		</>
	}
}

#[derive(PartialEq, Properties)]
struct ViewProperties {
	cells: UseStateHandle<CellsResult>,
	path: UseStateHandle<Option<Vec<u8>>>,
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
					<Cell cells={cells.clone()} y={i / width} x={i % width} path={props.path.clone()}/>
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
	representatives: UseStateHandle<Vec<usize>>,
	path: UseStateHandle<Option<Vec<u8>>>,
}

#[function_component(InputArea)]
fn input_area(props: &InputAreaProperties) -> Html {
	let textarea_ref = use_node_ref();
	let select_ref = use_node_ref();

	let textarea_value = use_state(|| AttrValue::from(INITIAL_CELLS));

	let oninput = {
		let textarea_ref = textarea_ref.clone();
		let select_ref = select_ref.clone();
		let textarea_value = textarea_value.clone();
		let cells_handle = props.cells.clone();
		let representatives_handle = props.representatives.clone();
		let path = props.path.clone();

		Callback::from(move |_| {
			let textarea = textarea_ref.cast::<HtmlTextAreaElement>();
			let select = select_ref.cast::<HtmlSelectElement>();

			if let Some((textarea, select)) = textarea.zip(select) {
				let value = textarea.value();
				let cells: CellsResult = TryFrom::try_from(value.as_ref());
				if let Ok(cells) = cells.as_ref() {
					representatives_handle.set(cells.find_routes());
					select.set_value("placeholder");
				}
				cells_handle.set(cells);
				textarea_value.set(AttrValue::from(value));
				path.set(None);
			}
		})
	};

	let onchange = {
		let node_ref = select_ref.clone();
		let cells_handle = props.cells.clone();
		let representatives_handle = props.representatives.clone();
		let path_handle = props.path.clone();

		Callback::from(move |_| {
			let select = node_ref.cast::<HtmlSelectElement>();

			if let Some(select) = select {
				let index: usize = select.value().parse().unwrap();
				let v = representatives_handle[index];
				path_handle.set(cells_handle.as_ref().unwrap().trace(v).into());
			}
		})
	};

	let textarea_class = props.cells.as_ref().map_or("error", |_| "");

	html! {
		<div class="input_area">
			<textarea class={textarea_class} ref={textarea_ref} rows={10} columns={32} oninput={oninput} spellcheck="false" value={(*textarea_value).clone()}/>
			<div>{"見つかったパス: "}{props.representatives.len()}</div>
			<select ref={select_ref} onchange={onchange} disabled={props.representatives.len() == 0}>
			if props.representatives.len() > 0 {
				<option hidden=true value="placeholder" selected={true}>{"選択してください"}</option>
				{(0..props.representatives.len()).map(|i| html! {
					<option value={format!("{i}")}>{'#'}{i + 1}</option>
				}).collect::<Html>()}
			}
			</select>
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
	let cells: UseStateHandle<CellsResult> = use_state(|| TryFrom::try_from(INITIAL_CELLS));
	let representatives = use_state(|| cells.clone().as_ref().unwrap().find_routes());
	let path = use_state(|| None);

	html! {
		<div class="bouncy_walls">
			<View cells={cells.clone()} path={path.clone()}/>
			<InputArea cells={cells} representatives={representatives.clone()} path={path}/>
		</div>
	}
}
