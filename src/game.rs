use once_cell::sync::Lazy;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

static TOPICS: Lazy<Vec<Vec<String>>> = Lazy::new(|| {
    let json = (|| {
        let path = std::env::args().nth(1)?;
        std::fs::read_to_string(path).ok()
    })();
    match json {
        Some(j) => serde_json::from_str(&j),
        None => serde_json::from_str(include_str!("../topics.json")),
    }
    .expect("Invalid JSON provided")
});

pub static GAME_MANAGER: Lazy<std::sync::RwLock<GameManager>> = Lazy::new(|| {
    std::sync::RwLock::new(GameManager {
        games: Default::default(),
        next_index: 0,
    })
});

pub type SharedGame = std::sync::Arc<std::sync::Mutex<Game>>;

pub struct GameManager {
    games: std::collections::HashMap<usize, SharedGame>,
    next_index: usize,
}
impl GameManager {
    pub fn create_game(&mut self) -> (SharedGame, usize) {
        let current_index = self.next_index;
        self.next_index += 1;
        let game = std::sync::Arc::new(std::sync::Mutex::new(Game { topics: draw() }));
        self.games.insert(current_index, game.clone());
        (game, current_index)
    }
}

pub struct Game {
    topics: [String; 2],
}

fn draw() -> [String; 2] {
    let mut rng = thread_rng();
    let mut t = vec![];
    while t.len() != 2 {
        t = TOPICS
            .choose(&mut rng)
            .expect("Empty topics")
            .iter()
            .filter(|x| x.len() < 18)
            .choose_multiple(&mut rng, 2);
    }
    [t[0].clone(), t[1].clone()]
}
