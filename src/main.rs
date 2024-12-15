use hot_reload::Game;

fn main() {
    let game = Game { tick: 0 };
    engine::run_game::<Game>(game);
}
