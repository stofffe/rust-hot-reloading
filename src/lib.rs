mod app;

#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
use std::{path::Path, sync::mpsc, time::Duration};

use dlopen::wrapper::{Container, WrapperApi};
use notify::Watcher;

pub trait GameCallbacks {
    fn start(&mut self);
    fn update(&mut self);
    fn end(&mut self);
}

#[derive(WrapperApi)]
struct DllApi {
    create_game: fn() -> Game,
    start: fn(game: &mut Game),
    update: fn(game: &mut Game),
    end: fn(game: &mut Game),
}

// must match Game in dynamic library
pub struct Game {
    pub current_tick: i32,
}

/// Wrapper for Game + dll reloading
pub struct GameWrapper {
    pub game: Game,
    pub dll: Container<DllApi>,
    pub dll_watcher: notify::FsEventWatcher,
    pub dll_change_channel: mpsc::Receiver<Result<notify::Event, notify::Error>>,
}

impl GameCallbacks for GameWrapper {
    fn start(&mut self) {
        self.dll.start(&mut self.game);
    }

    fn update(&mut self) {
        self.dll.update(&mut self.game);
    }

    fn end(&mut self) {
        self.dll.end(&mut self.game);
    }
}

impl GameWrapper {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let dll: Container<DllApi> = unsafe { Container::load("libgame.dylib") }
            .expect("Could not open library or load symbols");
        let game = dll.create_game();

        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(
                Path::new("libgame.dylib"),
                notify::RecursiveMode::NonRecursive,
            )
            .unwrap();

        Self {
            game,
            dll,
            dll_watcher: watcher,
            dll_change_channel: rx,
        }
    }

    /// checks if dll file has changed
    pub fn dll_changed(&self) -> bool {
        if let Ok(Ok(event)) = self.dll_change_channel.try_recv() {
            if let notify::EventKind::Modify(_) | notify::EventKind::Create(_) = event.kind {
                return true;
            }
        }
        false
    }

    /// reload dll file
    ///
    /// keep game state
    pub fn reload(&mut self) {
        self.dll = unsafe { Container::load("libgame.dylib") }
            .expect("Could not open library or load symbols");
    }

    /// reload dll file
    ///
    /// reset game state
    pub fn restart(&mut self) {
        self.dll = unsafe { Container::load("libgame.dylib") }
            .expect("Could not open library or load symbols");
        self.game = self.dll.create_game();
    }
}
