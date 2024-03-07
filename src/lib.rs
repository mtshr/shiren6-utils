mod bouncy_walls;
mod price_list;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Copy, PartialEq, Routable)]
enum Route {
	#[at("/")]
	Home,
	#[at("/bouncy-walls")]
	BouncyWalls,
	#[at("/price-list")]
	PriceList,
}

fn switch(routes: Route) -> Html {
	match routes {
		Route::Home => html! {},
		Route::BouncyWalls => html! { <bouncy_walls::BouncyWalls /> },
		Route::PriceList => html! { <price_list::PriceList /> },
	}
}

#[function_component(Menu)]
fn menu() -> Html {
	html! {
		<div>
			<div><Link<Route> to={Route::Home}>{"Top"}</Link<Route>></div>
			<div><Link<Route> to={Route::BouncyWalls}>{"ボヨヨン壁 成功パスサーチ"}</Link<Route>></div>
			<div><Link<Route> to={Route::PriceList}>{"鑑定補助 値段検索"}</Link<Route>></div>
		</div>
	}
}

#[function_component(App)]
fn app() -> Html {
	html! {
		<>
			<HashRouter>
				<Menu />
				<Switch<Route> render={switch} />
			</HashRouter>
			<div style="text-align: center">
				<a href={"https://github.com/mtshr"}>{"@mtshr"}</a>
			</div>
		</>
	}
}

#[wasm_bindgen]
pub fn run_app() {
	yew::Renderer::<App>::new().render();
}
