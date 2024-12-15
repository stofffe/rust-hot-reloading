#[macro_use]
extern crate dlopen_derive;

mod dll;

pub use dll::*;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub trait GameCallbacks {
    fn start(&mut self);
    fn update(&mut self);
    fn end(&mut self);
}

pub fn run_game<T: GameCallbacks>(callbacks: T) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    // NOTE: HOT_RELOAD
    let game = GameWrapper::<T>::new(callbacks);

    // app
    let mut app = WinitApp {
        window: None,
        game,
        // callbacks: game,
    };

    event_loop.run_app(&mut app).unwrap();
}

pub struct WinitApp<T: GameCallbacks> {
    pub window: Option<Window>,

    pub game: crate::GameWrapper<T>,
    // callbacks: T,
}

impl<T: GameCallbacks> ApplicationHandler for WinitApp<T> {
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
                        if code == KeyCode::KeyH {
                            println!("reload");
                            self.game.reload();
                        }
                        // NOTE: HOT_RELOAD
                        if code == KeyCode::KeyR {
                            println!("restart");
                            self.game.restart();
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // reload game
                // NOTE: HOT_RELOAD
                if self.game.dll_changed() {
                    self.game.reload();
                }

                self.game.update();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
