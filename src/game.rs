use once_cell::sync::Lazy;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{random, thread_rng};

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
    })
});

pub type SharedGame<'a> = std::sync::Arc<std::sync::Mutex<Game<'a>>>;
pub type GameId = u64;

pub struct GameManager<'a> {
    games: std::collections::HashMap<GameId, SharedGame<'a>>,
}

impl<'a> GameManager<'a> {
    fn game_exists(&self, id: GameId) -> bool {
        self.games.contains_key(&id)
    }

    pub fn create_game(&mut self) -> (SharedGame<'a>, GameId) {
        let mut index = random();
        while self.game_exists(index) {
            index = random();
        }

        let game = std::sync::Arc::new(std::sync::Mutex::new(Game {
            topics: draw(),
            players: vec![],
            wolf: None,
        }));
        self.games.insert(index, game.clone());
        (game, index)
    }
    pub fn get_game(&self, id: GameId) -> Option<SharedGame<'a>> {
        self.games.get(&id).cloned()
    }
    pub fn delete_game(&mut self, id: GameId) {
        self.games.remove(&id);
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
