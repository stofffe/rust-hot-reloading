use hot_reload::Game;

fn main() {
    let game = hot_reload::Game::new();
    engine::run_game::<Game>(game);
}
