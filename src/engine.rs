use std::collections::hash_map::Entry;
use std::collections::HashMap;
use vampirc_uci::UciOptionConfig;
use crate::constants::*;
use crate::constants::Move;
use crate::uci::OptionValue;

pub struct Engine {
	trans_table: Vec::<TEntry>,
	option_configs: HashMap<String, UciOptionConfig>,
	options: HashMap<String,OptionValue>
}

impl Engine {
	pub fn init() -> Engine {
		
		let options_configs = HashMap::from([
			(String::from("Hash"), UciOptionConfig::Spin {
				name: String::from("Hash"),
				default: Some(1024),
				min: Some(512),
				max: Some(16384)
			}),
			(String::from("Ponder"),UciOptionConfig::Check {
				name: String::from("Ponder"),
				default: Some(false),
			})
		]);

		let mut options = HashMap::<String,OptionValue>::new();

		for (_key, value) in options_configs.iter() {
			match value {
				UciOptionConfig::Spin {name, default, min, ..} => {
					let mut val:i64 = 0;
					if default.is_some() {
						val = default.unwrap();
					} else if min.is_some() {
						val = min.unwrap();
					}
					options.insert((*name).clone(), OptionValue::Spin(val));
				},
				UciOptionConfig::Check {name, default} => {
					let mut val:bool = false;
					if default.is_some() {
						val = default.unwrap();
					}
					options.insert((*name).clone(), OptionValue::Check(val));
				}
				_ => ()
			}
		}

		Engine {
			trans_table: Vec::<TEntry>::with_capacity(0),
			option_configs: options_configs,
			options: options
		}
	}

	pub fn go() -> Move {
		todo!()
	}

	pub fn set_option(&mut self, name: String, value: Option<String>) -> bool {
		if let Entry::Occupied(o) = self.options.entry(name.clone()) {
			match o.get() {
				OptionValue::Spin(_) => {
					if value.is_some() {
						self.options.insert(name, OptionValue::Spin(value.unwrap().parse::<i64>().unwrap()));
						return true
					}
					return false
				},
				OptionValue::Check(_) => {
					if value.is_some() {
						self.options.insert(name, OptionValue::Check(value.unwrap().parse::<bool>().unwrap()));
						return true
					}
					return false
				},
				_ => return false
			}
		} else {
			return false;
		}
	}

	pub fn get_options(&self) -> &HashMap<String, UciOptionConfig> {
		&self.option_configs
	}

	pub fn init_ttable(&mut self) -> bool{
		if let Entry::Occupied(o) = self.options.entry("Hash".to_string()) {
			if let OptionValue::Spin(cap) = o.get() {
				self.trans_table = Vec::<TEntry>::with_capacity(*cap as usize/SIZE_TENTRY);
				return true;
			} else {
				return false;
			}
		} else {
			return false;
		}
	}

	pub fn print_options(&self) {
		println!("{:?}", self.options);
	}
}