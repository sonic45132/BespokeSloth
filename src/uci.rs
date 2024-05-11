use crate::engine::Engine;
use crate::helpers::*;
use std::sync::mpsc;
use std::thread;
use std::io::{self, BufRead};
use vampirc_uci::parse_one;
use vampirc_uci::{UciMessage,UciOptionConfig};

#[derive(Debug)]
pub enum OptionValue {
	Check {
        name: String,
        val: bool,
    },
    Spin {
        name: String,
        val: i64
    },
    Combo {
        name: String,
        val: String
    },
    String {
        name: String,
        val: String
    }
}



//Sets up channels for passing data between control and compute threads.
pub fn setup_uci() {
	let (uci_tx, uci_rx) = mpsc::channel::<UciMessage>();

	thread::spawn(move || {
		for line in io::stdin().lock().lines() {
			let msg: UciMessage = parse_one(&line.unwrap());
			println!("info string Received message: {}", msg);
			uci_tx.send(msg).unwrap();
		}
	});

	uci_control(uci_rx);
}

fn uci_control(uci_rx: mpsc::Receiver<UciMessage>) {

	//Wait for start uci message
	wait_for_uci(&uci_rx);

	//Send engine id info
	send_uci_id();

	//Build engine option list
	let options = vec![
		UciOptionConfig::Spin {
			name: String::from("Hash"),
			default: Some(1024),
			min: Some(512),
			max: Some(16384)
		},
		UciOptionConfig::Check {
			name: String::from("Ponder"),
			default: Some(false),
		}, 
		UciOptionConfig::Check {
			name: String::from("Testing"),
			default: Some(true),
		},
		UciOptionConfig::Spin {
			name: String::from("Test2"),
			default: Some(123),
			min: Some(0),
			max: Some(1337),
		}
	];

	//Generate option values from option list
	let mut option_values = convert_to_values(&options);

	//Send available options
	send_options(&options);

	//Send uciok
	println!("{}",UciMessage::UciOk);

	loop {
		let msg = get_message_blocking(&uci_rx).unwrap();
		match msg {
			UciMessage::IsReady => break,
			UciMessage::SetOption { name, value } => {
				//println!("Got set option");
				let option = find_mut_option(&mut option_values, name);
				match option {
					Some(i) => {
						//println!("Found option");
						match i {
							OptionValue::Check{ ref mut val, .. } => {
								*val = match value.unwrap_or(String::from("")).as_ref() {
									"true" => true,
									"false" => false,
									_ => false
								};
							},
							OptionValue::Spin{ ref mut val, .. } => {
								*val = value.unwrap_or(String::from("0")).parse::<i64>().unwrap();
							},
							_ => println!("Option type not supported")
						}
					}
					None => ()//println!("Didnt find option")
				}
			},
			UciMessage::Quit => std::process::exit(0),
			_ => ()
		}
	}

	println!("info string Option Values: {:?}", option_values);

	let _engine = Engine::init(option_values);

	send_uci_message(UciMessage::ReadyOk);

	loop {
		let msg = get_message_blocking(&uci_rx).unwrap();
		match msg {
			UciMessage::Quit => std::process::exit(0),
			_ => ()
		}
	}
}

//Waits forever for start UCI message
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

	send_uci_message(name);
	send_uci_message(author);
}

fn send_options(opts: &Vec<UciOptionConfig>) {
	for option in opts.iter() {
		let tmp = option.clone();
		send_uci_message(UciMessage::Option(tmp));
	}
}

fn send_uci_message(msg: UciMessage) {
	println!("{}", msg);
}

fn convert_to_values(opts: &Vec<UciOptionConfig>) -> Vec<OptionValue> {
	let mut vals = Vec::new();
	for option in opts.iter() {
		match option {
			UciOptionConfig::Check { name, default} => {
				vals.push(
					OptionValue::Check {
						name: name.to_string(),
						val: default.unwrap_or(false)
					}
				);
			},
			UciOptionConfig::Spin { name, default, ..} => {
				vals.push(
					OptionValue::Spin {
						name: name.to_string(),
						val: default.unwrap_or(0)
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