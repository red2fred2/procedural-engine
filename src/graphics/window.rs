use std::sync::mpsc::{Sender, Receiver};

use tao::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}};

use super::frame_buffer::FrameBuffer;

/**
 * Descibes a window that can be rendered to.
 */
 pub struct WindowInput {
	pub frame_buffer: FrameBuffer
}

pub struct WindowOutput {
	/// Time since the last frame in microseconds
	pub dt: Option<u128>
}

pub fn create_window(input: Receiver<WindowInput>, output: Sender<WindowOutput>) {
	let event_loop = EventLoop::new();
	let window_builder = WindowBuilder::new();

	let window = window_builder.build(&event_loop).unwrap();

	let mut dt_timer = std::time::Instant::now();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				println!("The close button was pressed; stopping");
				*control_flow = ControlFlow::Exit
			},
			Event::MainEventsCleared => {
				// Application update code.

				// Queue a RedrawRequested event.
				//
				// You only need to call this if you've determined that you need to redraw, in
				// applications which do not always need to. Applications that redraw continuously
				// can just render here instead.
				window.request_redraw();
			},
			Event::RedrawRequested(_) => {
				// Redraw the application.
				//
				// It's preferable for applications that do not render continuously to render in
				// this event rather than in MainEventsCleared, since rendering in here allows
				// the program to gracefully handle redraws requested by the OS.
			},
			_ => ()
		}

		// Update dt timer
		let dt = Some(dt_timer.elapsed().as_micros());
		output.send(WindowOutput {dt});
		dt_timer = std::time::Instant::now();
	});
}
