use crate::helpers::*;
use std::sync::mpsc;
use std::thread;
use std::io::{self, BufRead};
use vampirc_uci::parse_one;
use vampirc_uci::{UciMessage,UciOptionConfig};

pub enum OptionValue {
	Check {
        name: String,
        value: bool,
    },
    Spin {
        name: String,
        value: i64
    },
    Combo {
        name: String,
        value: String
    },
    String {
        name: String,
        value: String
    }
}



//Sets up channels for passing data between control and compute threads.
pub fn setup_uci() {
	let (uci_tx, uci_rx) = mpsc::channel::<UciMessage>();

	thread::spawn(move || {
		for line in io::stdin().lock().lines() {
			let msg: UciMessage = parse_one(&line.unwrap());
			println!("Received message: {}", msg);
			uci_tx.send(msg).unwrap();
		}
	});

	uci_control(uci_rx);
}

fn uci_control(uci_rx: mpsc::Receiver<UciMessage>) {
	//Wait for beginning uci message
	wait_for_uci(&uci_rx);
	send_uci_id();

	let options = vec![
		UciOptionConfig::Spin {
			name: "Hash".to_string(),
			default: Some(1024),
			min: Some(512),
			max: Some(16384)
		},
		UciOptionConfig::Check {
			name: "Ponder".to_string(),
			default: Some(false),
		}
	];

	let _option_values = convert_to_values(&options);
	send_options(&options);

	println!("{}",UciMessage::UciOk);


}

fn wait_for_uci(uci_rx:&mpsc::Receiver<UciMessage>) {
	loop {
		let message = get_message_blocking(&uci_rx).unwrap();

		match message {
			UciMessage::Uci => break,
			_ => continue
		};
	}
}

fn send_uci_id() {
	let name = UciMessage::id_name("Bespoke Sloth");
	let author = UciMessage::id_author("Neonstatic");

	println!("{}", name);
	println!("{}", author);
}

fn send_options(opts: &Vec<UciOptionConfig>) {
	for option in opts.iter() {
		let tmp = option.clone();
		println!("{}", UciMessage::Option(tmp));
	}
}

fn convert_to_values(opts: &Vec<UciOptionConfig>) -> Vec<OptionValue> {
	let mut vals = Vec::new();
	for option in opts.iter() {
		match option {
			UciOptionConfig::Check { name, default} => {
				vals.push(
					OptionValue::Check {
						name: name.to_string(),
						value: default.unwrap_or(false)
					}
				);
			},
			UciOptionConfig::Spin { name, default, ..} => {
				vals.push(
					OptionValue::Spin {
						name: name.to_string(),
						value: default.unwrap_or(0)
					}
				);
			},
			_ => todo!(),
		}
	}
	vals
}

fn get_message_blocking(uci_rx:&mpsc::Receiver<UciMessage>) -> Option<UciMessage> {
	match uci_rx.recv() {
		Ok(msg) => Some(msg),
		Err(_e) => None
	}
}

fn get_message(uci_rx:mpsc::Receiver<UciMessage>) -> Option<UciMessage> {
	match uci_rx.try_recv() {
		Ok(msg) => Some(msg),
		Err(_e) => None
	}
}