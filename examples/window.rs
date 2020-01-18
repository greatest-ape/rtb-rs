#![allow(deprecated)]

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use winit::window::{WindowBuilder};


fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("winit window")
        .build(&event_loop)
        .unwrap();

    let test_string = "test_string";

    let raw_window_handle = match window.raw_window_handle(){
        RawWindowHandle::MacOS(h) => h.ns_window,
        RawWindowHandle::Windows(h) => h.hwnd, // FIXME: correct?
        RawWindowHandle::XcbHandle(h) => h.window, // FIXME: correct?
        RawWindowHandle::XlibHandle(h) => h.window, // FIXME: correct?
        _ => unimplemented!(),
    } as *mut std::ffi::c_void;

    let _ = rtb_rs::Window::attach(
        raw_window_handle,
        move |event| {
            println!("{:?}: {}", event, test_string);
        },
    );

    event_loop.run(move |event, _, control_flow| {
       *control_flow = ControlFlow::Poll;

       match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
