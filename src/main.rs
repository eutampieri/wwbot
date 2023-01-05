use game::GAME_MANAGER;

mod game;

fn main() {
    GAME_MANAGER.write().unwrap().create_game();
}
