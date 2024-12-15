use engine::Callbacks;

#[repr(C)]
#[derive(Clone, Default)]
pub struct Game {}

impl Game {
    #[no_mangle]
    pub fn new() -> Self {
        Self {}
    }
}

impl Callbacks for Game {
    #[no_mangle]
    fn init(&mut self) {
        println!("init")
    }

    #[no_mangle]
    fn update(&mut self) {
        // println!("update");
    }

    #[no_mangle]
    fn render(&mut self) {
        println!("render");
    }

    #[no_mangle]
    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        println!("resize {:?}", size)
    }
}

