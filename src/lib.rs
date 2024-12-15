use engine::GameCallbacks;

#[repr(C)]
#[derive(Clone, Default)]
pub struct Game {
    pub tick: i32,
}

#[no_mangle]
pub fn create_game() -> Game {
    Game::default()
}

impl GameCallbacks for Game {
    #[no_mangle]
    fn start(&mut self) {
        println!("start")
    }

    #[no_mangle]
    fn update(&mut self) {
        self.tick += 1;
        // println!("tick {}", self.tick);
    }

    #[no_mangle]
    fn end(&mut self) {
        println!("end")
    }
}
