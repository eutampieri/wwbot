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

pub type SharedGame<'a> = std::sync::Arc<std::sync::Mutex<Game<'a>>>;

pub struct GameManager<'a> {
    games: std::collections::HashMap<usize, SharedGame<'a>>,
    next_index: usize,
}
impl<'a> GameManager<'a> {
    pub fn create_game(&mut self) -> (SharedGame<'a>, usize) {
        let current_index = self.next_index;
        self.next_index += 1;
        let game = std::sync::Arc::new(std::sync::Mutex::new(Game {
            topics: draw(),
            players: vec![],
            wolf: None,
        }));
        self.games.insert(current_index, game.clone());
        (game, current_index)
    }
}

pub struct Game<'a> {
    topics: [String; 2],
    players: Vec<String>,
    wolf: Option<&'a str>,
}

impl<'a> Game<'a> {
    pub fn game_started(&self) -> bool {
        self.wolf.is_some()
    }

    pub fn add_player(&mut self, player: String) -> Result<(), &'static str> {
        if self.game_started() {
            Err("Game was already started!")
        } else {
            self.players.push(player);
            Ok(())
        }
    }

    pub fn start_game(&'a mut self) -> Result<(), &'static str> {
        if self.players.len() < 3 {
            return Err("Game with less than three players cannot be started");
        }
        let mut rng = thread_rng();
        let wolf = self.players.choose(&mut rng).unwrap();
        self.wolf = Some(wolf.as_str());
        Ok(())
    }
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
