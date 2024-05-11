use std::collections::HashMap;
use crate::engine::Engine;
use std::sync::mpsc;
use std::thread;
use std::io::{self, BufRead};
use vampirc_uci::parse_one;
use vampirc_uci::{UciMessage,UciOptionConfig};

#[derive(Debug)]
pub enum OptionValue {
	Check(bool),
    Spin(i64),
    Combo(String),
    String(String)
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

	//Build engine
	let mut engine = Engine::init();

	//Generate option values from option list
	let options = engine.get_options();

	//Send available options
	send_options(&options);

	//Send uciok
	println!("{}",UciMessage::UciOk);

	loop {
		let msg = get_message_blocking(&uci_rx).unwrap();
		match msg {
			UciMessage::IsReady => break,
			UciMessage::SetOption { name, value } => {
				println!("Got set option");
				engine.set_option(name, value);
			},
			UciMessage::Quit => std::process::exit(0),
			_ => ()
		}
	}

	if !engine.init_ttable() {
		println!("info Failed to init transposition table. Exiting!");
		std::process::exit(10);
	}

	engine.print_options();

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

fn send_options(opts: &HashMap<String,UciOptionConfig>) {
	for (_key, val) in opts.iter() {
		println!("{}", *val);
	}
}

fn send_uci_message(msg: UciMessage) {
	println!("{}", msg);
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