use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

use ash::{vk, Entry};
use std::ffi::CString;
use std::ptr;

mod core;

const WINDOW_TITLE: &'static str = "00.Base Code";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;


struct VulkanApp;

impl VulkanApp {

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn main_loop(event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {

            if let Event::WindowEvent { event, .. } = event {
                if WindowEvent::CloseRequested == event {
                    *control_flow = ControlFlow::Exit;
                } else if let WindowEvent::KeyboardInput {input, ..  } = event {
                    let KeyboardInput { virtual_keycode, state, .. } = input;
                    if (Some(VirtualKeyCode::Escape), ElementState::Pressed) == (virtual_keycode, state) {
                        dbg!();
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
        })
    }
}

fn main() {

    let entry = unsafe { Entry::new().unwrap() };
    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);

    let _ctx: core::Context = core::Context::new(&entry, "Ast", vk::make_api_version(0, 0, 1, 0));

    VulkanApp::main_loop(event_loop);
}
