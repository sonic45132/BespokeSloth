use crate::helpers::*;
use std::sync::mpsc;
use std::thread;


//Sets up channels for passing data between control and compute threads.
pub fn run_uci() {
	let (commandtx, commandrx) = mpsc::channel::<String>();
	let (resulttx, resultrx) = mpsc::channel::<String>();

	thread::spawn(move || {
		uci_control(commandtx, resultrx);
	});

	compute_control(commandrx, resulttx);
}

//Compute thread main function. Handles uci logic and controls main compute thread.
fn uci_control(commandtx: mpsc::Sender<String>, resultrx: mpsc::Receiver<String>) {

	//Setup and spawn the thread to handle blocking stdin reading
	let (stdin_tx, stdin_rx) = mpsc::channel::<String>();
	thread::spawn(move || {
		loop {
			let line = read_line();
			stdin_tx.send(line);
		}
	});

	loop {
		let stdin_rx_result = stdin_rx.try_recv();
		let compute_info_result = resultrx.try_recv();

		let stdin_line = match stdin_rx_result {
			Ok(line) => line,
			Err(e) => String::from("")
		};

		if stdin_line != "" {
			commandtx.send(stdin_line).unwrap();
		}
	}
}

fn compute_control(_commandrx: mpsc::Sender<String>, _resulttx: mpsc::Receiver<String>) {

}