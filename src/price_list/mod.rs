mod items;

use std::num::ParseIntError;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct InputProps {
	price: UseStateHandle<Result<Option<u16>, ParseIntError>>,
}

#[function_component(Input)]
fn input(props: &InputProps) -> Html {
	let input_ref = use_node_ref();

	let oninput = {
		let input_ref = input_ref.clone();
		let price_handle = props.price.clone();

		Callback::from(move |_| {
			let input = input_ref.cast::<HtmlInputElement>();

			if let Some(input) = input {
				let value = input.value();
				let price = if value.is_empty() {
					Ok(None)
				} else {
					value.parse::<u16>().map(|price| Some(price))
				};
				price_handle.set(price);
			}
		})
	};

	html! {
		<div>
			<label class="input_label">{"値段:"}</label>
			<input oninput={oninput} ref={input_ref} inputmode="numeric" maxlength="7"/>
		</div>
	}
}

#[derive(Clone, PartialEq, Properties)]
struct ListProps {
	price: UseStateHandle<Result<Option<u16>, ParseIntError>>,
}

#[function_component(List)]
fn list(props: &ListProps) -> Html {
	let price = (*props.price).clone();

	let mut bracelets: Vec<_> = items::filtered_bracelets(price.clone()).collect();
	if let Ok(&Some(price)) = price.as_ref() {
		bracelets.sort_by(|a, b| (b.buy == price).cmp(&(a.buy == price)));
	}
	let mut grasses: Vec<_> = items::filtered_grasses(price.clone()).collect();
	if let Ok(&Some(price)) = price.as_ref() {
		grasses.sort_by(|a, b| (b.buy == price).cmp(&(a.buy == price)));
	}
	let mut scrolls: Vec<_> = items::filtered_scrolls(price.clone()).collect();
	if let Ok(&Some(price)) = price.as_ref() {
		scrolls.sort_by(|a, b| (b.buy == price).cmp(&(a.buy == price)));
	}
	let mut staves: Vec<_> = items::filtered_staves(price.clone()).collect();
	if let Ok(&Some(price)) = price.as_ref() {
		staves.sort_by(|a, b| (b.buy == price).cmp(&(a.buy == price)));
	}
	let mut pots: Vec<_> = items::filtered_pots(price.clone()).collect();
	if let Ok(&Some(price)) = price.as_ref() {
		pots.sort_by(|a, b| (b.buy == price).cmp(&(a.buy == price)));
	}

	html! {
		<div class="container">
			<div class="category">{"腕輪"}</div>
			<table>
				if bracelets.len() > 0 {
					<thead>
						<tr>
							<th scope="col" colspan="2">{"名前"}</th>
							<th scope="col">{"買値"}</th>
							<th scope="col">{"売値"}</th>
						</tr>
					</thead>
					<tbody>
					{
						bracelets.into_iter().map(|item| html! {
							<tr>
								<th class="state">{item.state.get_state_str()}</th>
								<td class="name">{item.name}</td>
								<td>{item.buy}</td>
								<td>{item.sell}</td>
							</tr>
						}).collect::<Html>()
					}
					</tbody>
				}
			</table>
			<div class="category">{"草"}</div>
			<table>
				if grasses.len() > 0 {
					<thead>
						<tr>
							<th scope="col" colspan="2">{"名前"}</th>
							<th scope="col">{"買値"}</th>
							<th scope="col">{"売値"}</th>
						</tr>
					</thead>
					<tbody>
					{
						grasses.into_iter().map(|item| html! {
							<tr>
								<th class="state">{item.state.get_state_str()}</th>
								<td class="name">{item.name}</td>
								<td>{item.buy}</td>
								<td>{item.sell}</td>
							</tr>
						}).collect::<Html>()
					}
					</tbody>
				}
			</table>
			<div class="category">{"巻物"}</div>
			<table>
				if scrolls.len() > 0 {
					<thead>
						<tr>
							<th scope="col" colspan="2">{"名前"}</th>
							<th scope="col">{"買値"}</th>
							<th scope="col">{"売値"}</th>
						</tr>
					</thead>
					<tbody>
					{
						scrolls.into_iter().map(|item| html! {
							<tr>
								<th class="state">{item.state.get_state_str()}</th>
								<td class="name">{item.name}</td>
								<td>{item.buy}</td>
								<td>{item.sell}</td>
							</tr>
						}).collect::<Html>()
					}
					</tbody>
				}
			</table>
			<div class="category">{"杖"}</div>
			<table>
				if staves.len() > 0 {
					<thead>
						<tr>
							<th scope="col" colspan="2">{"名前"}</th>
							<th scope="col">{"回数"}</th>
							<th scope="col">{"買値"}</th>
							<th scope="col">{"売値"}</th>
						</tr>
					</thead>
					<tbody>
					{
						staves.into_iter().map(|item| html! {
							<tr>
								<th class="state">{item.state.get_state_str()}</th>
								<td class="name">{item.name}</td>
								<td>{item.count}</td>
								<td>{item.buy}</td>
								<td>{item.sell}</td>
							</tr>
						}).collect::<Html>()
					}
					</tbody>
				}
			</table>
			<div class="category">{"壺"}</div>
			<table>
				if pots.len() > 0 {
					<thead>
						<tr>
							<th scope="col" colspan="2">{"名前"}</th>
							<th scope="col">{"容量"}</th>
							<th scope="col">{"買値"}</th>
							<th scope="col">{"売値"}</th>
						</tr>
					</thead>
					<tbody>
					{
						pots.into_iter().map(|item| html! {
							<tr>
								<th class="state">{item.state.get_state_str()}</th>
								<td class="name">{item.name}</td>
								<td>{item.size}</td>
								<td>{item.buy}</td>
								<td>{item.sell}</td>
							</tr>
						}).collect::<Html>()
					}
					</tbody>
				}
			</table>
		</div>
	}
}

#[function_component(PriceList)]
pub fn price_list() -> Html {
	let price = use_state(|| Ok(None));

	html! {
		<div class="price_list">
			<Input price={price.clone()}/>
			<List price={price}/>
		</div>
	}
}
