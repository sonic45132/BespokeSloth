use vampirc_uci::UciOptionConfig;
use crate::helpers::find_option;
use crate::constants::TEntry;
use crate::constants::Move;
use crate::uci::OptionValue;

pub struct Engine {
	trans_table: Vec::<TEntry>,
	options: Vec<OptionValue>
}

pub enum Engineoption {
	Check {
		name: String,
		value: bool,
		default: Option<bool>
	},
	Spin {
		name: String,
		value: i64,
		default: Option<i64>,
		min: Option<i64>,
		max: Option<i64>
	}
} 

impl Engine {
	pub fn init(options:Vec<OptionValue>) -> Engine {
		let size = std::mem::size_of::<TEntry>();
		let hash_size = match find_option(&options, String::from("Hash")) {
			Some(x) => {
				match x {
					OptionValue::Spin{ val, ..} => *val,
					_ => 1024
				}
			},
			None => 1024
		};

		Engine {
			trans_table: Vec::<TEntry>::with_capacity(hash_size as usize/size),
			options: options
		}
	}

	pub fn go() -> Move {
		Move {
			start: 0,
			target: 0,
			castle: 0 
		}
	}

	pub fn set_option() -> bool {
		todo!()
	}

	pub fn get_options() -> Vec<UciOptionConfig> {
		todo!()
	}
}