mod bouncy_walls;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
	html! {
		<div style={"text-align: center"}>
			<bouncy_walls::BouncyWalls />
			<div>
				<a href={"https://github.com/mtshr"}>{"@mtshr"}</a>
			</div>
		</div>
	}
}

#[wasm_bindgen]
pub fn run_app() {
	yew::Renderer::<App>::new().render();
}
