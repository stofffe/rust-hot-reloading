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
    fn start(&mut self) {
        println!("start")
    }

    #[no_mangle]
    fn update(&mut self) {
        println!("update");
    }

    #[no_mangle]
    fn end(&mut self) {
        println!("end")
    }
}
