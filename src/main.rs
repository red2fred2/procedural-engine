use std::sync::mpsc::{Receiver, Sender};

use graphics::window::{WindowInput, WindowOutput};

mod graphics;

const STACK_SIZE: usize = 2 * 1024 * 1024;

/**
 * Since the window event loop demands to be on the main thread, we'll pop a
 * new thread off and run init as our "main"
 */
fn main() {
	let (window_input_tx, window_input_rx) = std::sync::mpsc::channel();
	let (window_output_tx, window_output_rx) = std::sync::mpsc::channel();

	std::thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || init(window_input_tx, window_output_rx))
        .unwrap();

	graphics::window::create_window(window_input_rx, window_output_tx);
}

/**
 * The actual main function after a window is created
 */
fn init(window_input: Sender<WindowInput>, window_output: Receiver<WindowOutput>) {

	// Just read off the frame dt to test
	loop {
		let window_dt = window_output.recv().unwrap().dt;
		match window_dt {
			Some(dt) => println!("Time since last window poll {} us", dt),
			_ => ()
		};
	}
}
