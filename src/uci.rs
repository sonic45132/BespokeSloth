use crate::helpers::*;
use std::sync::mpsc;
use std::thread;



pub fn setup_uci() {
	let (commandtx, commandrx) = mpsc::channel::<String>();
	let (resulttx, resultrx) = mpsc::channel::<String>();

	thread::spawn(move || {
        uci_input(commandtx, resultrx);
    });

    let received: String = commandrx.recv().unwrap();
    println!("Got: {}", received);
}

fn uci_input(commandtx: mpsc::Sender<String>, _resultrx: mpsc::Receiver<String>) {
	//while true {
		let line = read_line();
		commandtx.send(line).unwrap();
	//}
}

// fn uci_control(commandrx, resulttx) {

// }