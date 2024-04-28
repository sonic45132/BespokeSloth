use crate::helpers::find_option;
use crate::constants::TEntry;
use crate::constants::Move;
use crate::uci::OptionValue;

pub struct Engine {
	trans_table: Vec::<TEntry>,

}

impl Engine {
	pub fn init(options:Vec<OptionValue>) -> Engine {
		let size = std::mem::size_of::<TEntry>();
		let hash_size = match find_option(&options) {
			Some(x) => {
				match x {
					OptionValue::Spin{ value, ..} => *value,
					_ => 1024
				}
			},
			None => 1024
		};

		Engine {
			trans_table: Vec::<TEntry>::with_capacity(hash_size as usize/size)
		}
	}

	pub fn go() -> Move {
		Move {
			start: 0,
			target: 0,
			castle: 0 
		}
	}
}

