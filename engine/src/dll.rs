extern crate dlopen;

use std::{fs, path::Path, sync::mpsc};

use dlopen::wrapper::{Container, WrapperApi};
use notify::Watcher;

const DLL_NAME: &str = "hot_reload_0.dll";
const DLL_TARGET: &str = "target/debug/hot_reload.dll";

/// Dll Api for Callbacks
#[derive(WrapperApi)]
pub struct DllApi<T> {
    new: fn() -> T,
    init: fn(game: &mut T),
    update: fn(game: &mut T),
    render: fn(game: &mut T),
    resize: fn(game: &mut T, size: winit::dpi::PhysicalSize<u32>),
}

/// Wrapper for Game + dll reloading
pub struct DllCallbacks<T> {
    pub callbacks: T,
    pub dll: Container<DllApi<T>>,
    pub dll_watcher: notify::RecommendedWatcher,
    pub dll_change_channel: mpsc::Receiver<Result<notify::Event, notify::Error>>,

    pub i: usize,
}

impl<T> crate::Callbacks for DllCallbacks<T> {
    fn init(&mut self) {
        self.dll.init(&mut self.callbacks);
    }

    fn update(&mut self) {
        self.dll.update(&mut self.callbacks);
    }

    fn render(&mut self) {
        self.dll.render(&mut self.callbacks);
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.dll.resize(&mut self.callbacks, size);
    }
}

impl<T> DllCallbacks<T> {
    // NOTE: the callbacks are not used
    // since they will be loaded from the DLL
    pub fn new(_callbacks: T) -> Self {
        let i = 0;
        let dll: Container<DllApi<T>> = load_dll(i);

        // unsafe { Container::load(DLL_NAME) }.expect("Could not open library or load symbols");
        let game = dll.new();

        let (tx, rx) = mpsc::channel();

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        watcher
            .watch(Path::new(DLL_TARGET), notify::RecursiveMode::NonRecursive)
            .unwrap();

        Self {
            callbacks: game,
            dll,
            dll_watcher: watcher,
            dll_change_channel: rx,
            i,
        }
    }

    /// checks if dll file has changed
    pub fn dll_changed(&self) -> bool {
        if let Ok(Ok(event)) = self.dll_change_channel.try_recv() {
            println!("EVENT {event:?}");
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
        println!("--- hot_reload ---");
        // unload current
        self.i += 1;
        self.dll = load_dll(self.i);

        // let target = format_dll_name(self.i);
        //
        // // copy
        // fs::copy(DLL_TARGET, &target).expect("could not copy dll");

        // self.dll =
        //     unsafe { Container::load(&target) }.expect("Could not open library or load symbols");
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

fn load_dll<T>(i: usize) -> Container<DllApi<T>> {
    let target = format_dll_name(i);

    fs::copy(DLL_TARGET, &target).expect("could not copy dll");

    unsafe { Container::load(&target) }.expect("Could not open library or load symbols")
}

fn format_dll_name(i: usize) -> String {
    format!("hot_reload_{i}.dll")
}
