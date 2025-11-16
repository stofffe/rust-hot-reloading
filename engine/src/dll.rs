extern crate dlopen;

use std::{env, fs, path::PathBuf, sync::mpsc};

use dlopen::wrapper::{Container, WrapperApi};
use notify::Watcher;

fn format_dll_input() -> PathBuf {
    let current_exe = env::current_exe().expect("could not get current exe path");
    let folder_path = current_exe
        .parent()
        .expect("could not get current exe parent folder");
    let file_name = current_exe
        .file_stem()
        .expect("could not get current exe file stem");
    let file_name = file_name
        .to_str()
        .expect("could not convert os string to &str")
        .replace("-", "_");
    let path = PathBuf::new()
        .join(folder_path)
        .join(dlopen::utils::platform_file_name(file_name));

    println!("{path:?}");
    path
}

fn format_dll_output(flipflop: bool) -> PathBuf {
    let current_exe = env::current_exe().expect("could not get current exe path");
    let folder_path = current_exe
        .parent()
        .expect("could not get current exe parent folder");
    let file_name = current_exe
        .file_stem()
        .expect("could not get current exe file stem");
    let file_name = file_name
        .to_str()
        .expect("could not convert os string to &str")
        .replace("-", "_");
    let file_version = if flipflop { 0 } else { 1 };
    let file_name = format!("{file_name}_{file_version}");

    PathBuf::new()
        .join(folder_path)
        .join(dlopen::utils::platform_file_name(file_name))
}

fn load_dll<T>(flipflop: bool) -> Container<DllApi<T>> {
    let input = format_dll_input();
    let output = format_dll_output(flipflop);
    println!("input {input:?}");
    println!("output {output:?}");
    fs::copy(input, &output).expect("could not copy dll");

    unsafe { Container::load(output) }.expect("Could not open library or load symbols")
}

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

    pub flipflop: bool,
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
        let flipflop = true;
        let dll: Container<DllApi<T>> = load_dll(flipflop);

        let callbacks = dll.new();

        let (tx, rx) = mpsc::channel();

        let mut dll_watcher = notify::recommended_watcher(tx).unwrap();
        dll_watcher
            .watch(&format_dll_input(), notify::RecursiveMode::NonRecursive)
            .unwrap();

        Self {
            callbacks,
            dll,
            dll_watcher,
            dll_change_channel: rx,
            flipflop,
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
        self.flipflop = !self.flipflop;
        self.dll = load_dll(self.flipflop);
    }

    /// reload dll file
    ///
    /// reset game state
    pub fn hot_restart(&mut self) {
        self.hot_reload();
        self.callbacks = self.dll.new();
    }
}
