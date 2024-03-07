use std::{num::ParseIntError, sync::OnceLock};

const BRACELETS_CSV: &str = include_str!("./bracelets.csv");
const GRASSES_CSV: &str = include_str!("./grasses.csv");
const SCROLLS_CSV: &str = include_str!("./scrolls.csv");
const STAVES_CSV: &str = include_str!("./staves.csv");
const POTS_CSV: &str = include_str!("./pots.csv");

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemState {
	Normal,
	Blessed,
	Cursed,
}

impl ItemState {
	pub fn get_state_str(&self) -> &str {
		use ItemState::*;

		match self {
			Normal => "",
			Blessed => "🔔",
			Cursed => "💀",
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bracelet {
	pub name: &'static str,
	pub state: ItemState,
	pub buy: u16,
	pub sell: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Grass {
	pub name: &'static str,
	pub state: ItemState,
	pub buy: u16,
	pub sell: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Scroll {
	pub name: &'static str,
	pub state: ItemState,
	pub buy: u16,
	pub sell: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Staff {
	pub name: &'static str,
	pub state: ItemState,
	pub buy: u16,
	pub sell: u16,
	pub count: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pot {
	pub name: &'static str,
	pub state: ItemState,
	pub buy: u16,
	pub sell: u16,
	pub size: u16,
}

fn bracelets() -> &'static Vec<Bracelet> {
	static BRACELETS: OnceLock<Vec<Bracelet>> = OnceLock::new();
	BRACELETS.get_or_init(|| {
		let mut list = Vec::new();
		for line in BRACELETS_CSV.trim().lines() {
			let mut it = line.split(',');
			let name = it.next().unwrap();
			let buy: u16 = it.next().unwrap().parse().unwrap();
			let sell = buy * 2 / 5;
			list.push(Bracelet {
				name,
				state: ItemState::Normal,
				buy,
				sell,
			});
			list.push(Bracelet {
				name,
				state: ItemState::Cursed,
				buy: (buy as u32 * 87 / 100) as u16,
				sell: (sell as u32 * 87 / 100) as u16,
			});
		}
		list
	})
}

fn grasses() -> &'static Vec<Grass> {
	static GRASSES: OnceLock<Vec<Grass>> = OnceLock::new();
	GRASSES.get_or_init(|| {
		let mut list = Vec::new();
		for line in GRASSES_CSV.trim().lines() {
			let mut it = line.split(',');
			let name = it.next().unwrap();
			let buy: u16 = it.next().unwrap().parse().unwrap();
			let sell: u16 = it.next().unwrap().parse().unwrap();
			list.push(Grass {
				name,
				state: ItemState::Normal,
				buy,
				sell,
			});
			list.push(Grass {
				name,
				state: ItemState::Blessed,
				buy: buy * 2,
				sell: sell * 2,
			});
			list.push(Grass {
				name,
				state: ItemState::Cursed,
				buy: (buy as u32 * 87 / 100) as u16,
				sell: (sell as u32 * 87 / 100) as u16,
			});
		}
		list
	})
}

fn scrolls() -> &'static Vec<Scroll> {
	static SCROLLS: OnceLock<Vec<Scroll>> = OnceLock::new();
	SCROLLS.get_or_init(|| {
		let mut list = Vec::new();
		for line in SCROLLS_CSV.trim().lines() {
			let mut it = line.split(',');
			let name = it.next().unwrap();
			let buy: u16 = it.next().unwrap().parse().unwrap();
			let sell: u16 = buy * 2 / 5;
			list.push(Scroll {
				name,
				state: ItemState::Normal,
				buy,
				sell,
			});
			list.push(Scroll {
				name,
				state: ItemState::Blessed,
				buy: buy * 2,
				sell: sell * 2,
			});
			list.push(Scroll {
				name,
				state: ItemState::Cursed,
				buy: (buy as u32 * 87 / 100) as u16,
				sell: (sell as u32 * 87 / 100) as u16,
			});
		}
		list
	})
}

fn staves() -> &'static Vec<Staff> {
	static STAVES: OnceLock<Vec<Staff>> = OnceLock::new();
	STAVES.get_or_init(|| {
		let mut list = Vec::new();
		for line in STAVES_CSV.trim().lines() {
			let mut it = line.split(',');
			let name = it.next().unwrap();
			let buy: u16 = it.next().unwrap().parse().unwrap();
			let _min = it.next().unwrap();
			let max = it.next().unwrap().parse().unwrap();
			for count in 0..=max {
				let buy = buy + 100 * count;
				list.push(Staff {
					name,
					state: ItemState::Normal,
					buy,
					sell: buy * 2 / 5,
					count,
				});
				list.push(Staff {
					name,
					state: ItemState::Cursed,
					buy: (buy as u32 * 87 / 100) as u16,
					sell: (buy as u32 * 2 / 5 * 87 / 100) as u16,
					count,
				});
			}
		}
		list
	})
}

fn pots() -> &'static Vec<Pot> {
	static POTS: OnceLock<Vec<Pot>> = OnceLock::new();
	POTS.get_or_init(|| {
		let mut list = Vec::new();
		for line in POTS_CSV.trim().lines() {
			let mut it = line.split(',');
			let name = it.next().unwrap();
			let buy: u16 = it.next().unwrap().parse().unwrap();
			let _min = it.next().unwrap();
			let max = it.next().unwrap().parse().unwrap();
			for size in 0..=max {
				let buy = buy + 100 * size;
				list.push(Pot {
					name,
					state: ItemState::Normal,
					buy,
					sell: buy * 2 / 5,
					size,
				});
				list.push(Pot {
					name,
					state: ItemState::Cursed,
					buy: (buy as u32 * 87 / 100) as u16,
					sell: (buy as u32 * 2 / 5 * 87 / 100) as u16,
					size,
				});
			}
		}
		list
	})
}

pub fn filtered_bracelets(
	price: Result<Option<u16>, ParseIntError>,
) -> impl Iterator<Item = &'static Bracelet> {
	let bracelets = bracelets();
	bracelets.iter().filter(move |item| {
		price
			.as_ref()
			.is_ok_and(|price| price.map_or(true, |price| item.buy == price || item.sell == price))
	})
}

pub fn filtered_grasses(
	price: Result<Option<u16>, ParseIntError>,
) -> impl Iterator<Item = &'static Grass> {
	let grasses = grasses();
	grasses.iter().filter(move |item| {
		price
			.as_ref()
			.is_ok_and(|price| price.map_or(true, |price| item.buy == price || item.sell == price))
	})
}

pub fn filtered_scrolls(
	price: Result<Option<u16>, ParseIntError>,
) -> impl Iterator<Item = &'static Scroll> {
	let scrolls = scrolls();
	scrolls.iter().filter(move |item| {
		price
			.as_ref()
			.is_ok_and(|price| price.map_or(true, |price| item.buy == price || item.sell == price))
	})
}

pub fn filtered_staves(
	price: Result<Option<u16>, ParseIntError>,
) -> impl Iterator<Item = &'static Staff> {
	let staves = staves();
	staves.iter().filter(move |item| {
		price
			.as_ref()
			.is_ok_and(|price| price.map_or(true, |price| item.buy == price || item.sell == price))
	})
}

pub fn filtered_pots(
	price: Result<Option<u16>, ParseIntError>,
) -> impl Iterator<Item = &'static Pot> {
	let pots = pots();
	pots.iter().filter(move |item| {
		price
			.as_ref()
			.is_ok_and(|price| price.map_or(true, |price| item.buy == price || item.sell == price))
	})
}
