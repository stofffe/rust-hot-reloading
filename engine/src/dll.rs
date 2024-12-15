extern crate dlopen;

use std::{path::Path, sync::mpsc};

use dlopen::wrapper::{Container, WrapperApi};
use notify::Watcher;

const DLL_NAME: &str = "libhot_reload.dylib";

/// Dll Api for Callbacks
#[derive(WrapperApi)]
pub struct DllApi<T> {
    new: fn() -> T,
    start: fn(game: &mut T),
    update: fn(game: &mut T),
    end: fn(game: &mut T),
}

/// Wrapper for Game + dll reloading
pub struct DllCallbacks<T> {
    pub callbacks: T,
    pub dll: Container<DllApi<T>>,
    pub dll_watcher: notify::FsEventWatcher,
    pub dll_change_channel: mpsc::Receiver<Result<notify::Event, notify::Error>>,
}

impl<T> crate::Callbacks for DllCallbacks<T> {
    fn start(&mut self) {
        self.dll.start(&mut self.callbacks);
    }

    fn update(&mut self) {
        self.dll.update(&mut self.callbacks);
    }

    fn end(&mut self) {
        self.dll.end(&mut self.callbacks);
    }
}

impl<T> DllCallbacks<T> {
    // NOTE: the callbacks are not used
    // since they will be loaded from the DLL
    pub fn new(_callbacks: T) -> Self {
        let dll: Container<DllApi<T>> =
            unsafe { Container::load(DLL_NAME) }.expect("Could not open library or load symbols");
        let game = dll.new();

        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(Path::new(DLL_NAME), notify::RecursiveMode::NonRecursive)
            .unwrap();

        Self {
            callbacks: game,
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
    pub fn hot_reload(&mut self) {
        self.dll =
            unsafe { Container::load(DLL_NAME) }.expect("Could not open library or load symbols");
    }

    /// reload dll file
    ///
    /// reset game state
    pub fn hot_restart(&mut self) {
        self.dll =
            unsafe { Container::load(DLL_NAME) }.expect("Could not open library or load symbols");
        self.callbacks = self.dll.new();
    }
}
