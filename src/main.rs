use hot_reload::GameCallbacks;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

struct WinitApp {
    window: Option<Window>,

    game: hot_reload::GameWrapper,
}

impl ApplicationHandler for WinitApp {
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
                        if code == KeyCode::KeyH {
                            println!("reload");
                            self.game.reload();
                        }
                        if code == KeyCode::KeyR {
                            println!("restart");
                            self.game.restart();
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // reload game
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

fn main() {
    let game = hot_reload::GameWrapper::new();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    // app
    let mut app = WinitApp { window: None, game };

    event_loop.run_app(&mut app).unwrap();
}
