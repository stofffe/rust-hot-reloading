#[macro_use]
extern crate dlopen_derive;

#[cfg(debug_assertions)]
mod dll;
#[cfg(debug_assertions)]
pub use dll::*;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub trait Callbacks {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>);
}

pub fn run_game<T: Callbacks>(callbacks: T) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    // NOTE: HOT_RELOAD
    #[cfg(debug_assertions)]
    let callbacks = DllCallbacks::new(callbacks);

    // app
    let mut app = App {
        window: None,
        callbacks,
    };

    event_loop.run_app(&mut app).unwrap();
}

pub struct App<T: Callbacks> {
    pub window: Option<Window>,

    #[cfg(debug_assertions)]
    pub callbacks: crate::DllCallbacks<T>,

    #[cfg(not(debug_assertions))]
    pub callbacks: T,
}

impl<T: Callbacks> ApplicationHandler for App<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state.is_pressed() {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        // NOTE: HOT_RELOAD
                        #[cfg(debug_assertions)]
                        if code == KeyCode::KeyH {
                            println!("reload");
                            self.callbacks.hot_reload();
                        }
                        // NOTE: HOT_RELOAD
                        #[cfg(debug_assertions)]
                        if code == KeyCode::KeyR {
                            println!("restart");
                            self.callbacks.hot_restart();
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                self.callbacks.resize(size);
            }
            WindowEvent::RedrawRequested => {
                // reload callbacks
                // NOTE: HOT_RELOAD
                #[cfg(debug_assertions)]
                if self.callbacks.dll_changed() {
                    self.callbacks.hot_reload();
                }

                self.callbacks.update();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
