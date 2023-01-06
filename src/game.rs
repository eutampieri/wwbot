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
const GAME_DURATION: std::time::Duration = std::time::Duration::from_secs(300);

pub static GAME_MANAGER: Lazy<std::sync::RwLock<GameManager>> = Lazy::new(|| {
    std::sync::RwLock::new(GameManager {
        games: Default::default(),
    })
});

pub type SharedGame = std::sync::Arc<std::sync::Mutex<Game>>;
pub type GameId = u64;

pub struct GameManager {
    games: std::collections::HashMap<GameId, SharedGame>,
}

impl GameManager {
    fn game_exists(&self, id: GameId) -> bool {
        self.games.contains_key(&id)
    }

    pub fn create_game(&mut self) -> (SharedGame, GameId) {
        let mut index = random();
        while self.game_exists(index) {
            index = random();
        }

        let game = std::sync::Arc::new(std::sync::Mutex::new(Game {
            topics: draw(),
            players: vec![],
            wolf: None,
            start_time: None,
            votes: vec![],
        }));
        self.games.insert(index, game.clone());
        (game, index)
    }
    pub fn get_game(&self, id: GameId) -> Option<SharedGame> {
        self.games.get(&id).cloned()
    }
    pub fn delete_game(&mut self, id: GameId) {
        self.games.remove(&id);
    }
}

#[derive(PartialEq, Eq)]
pub enum GameStatus {
    NotStarted,
    DiscussionTime,
    VotingTime,
    RunEnded,
}

pub struct Game {
    topics: [String; 2],
    players: Vec<String>,
    wolf: Option<usize>,
    start_time: Option<std::time::Instant>,
    votes: Vec<Option<usize>>,
}

impl Game {
    pub fn add_player(&mut self, player: String) -> Result<(), &'static str> {
        if self.get_status() == GameStatus::NotStarted {
            self.players.push(player);
            Ok(())
        } else {
            Err("Game was already started!")
        }
    }

    pub fn remove_player(&mut self, player: String) -> Result<(), &'static str> {
        if self.get_status() == GameStatus::NotStarted {
            let index = self
                .players
                .iter()
                .position(|x| *x == player)
                .ok_or("Player not found")?;
            self.players.remove(index);
            Ok(())
        } else {
            Err("Game was already started!")
        }
    }

    pub fn start_game(&mut self) -> Result<(), &'static str> {
        if self.players.len() < 3 {
            return Err("Game with less than three players cannot be started");
        }
        let mut rng = thread_rng();
        let wolf = self.players.iter().enumerate().choose(&mut rng).unwrap().0;
        self.wolf = Some(wolf);
        self.start_time = Some(std::time::Instant::now());
        self.votes.resize(self.players.len(), None);
        Ok(())
    }

    pub fn get_status(&self) -> GameStatus {
        if self.votes.len() != self.players.len() && self.votes.iter().all(|x| x.is_some()) {
            GameStatus::RunEnded
        } else if self
            .start_time
            .map(|x| std::time::Instant::now() - x > GAME_DURATION)
            .unwrap_or_default()
        {
            GameStatus::VotingTime
        } else if self.wolf.is_some() && self.start_time.is_some() {
            GameStatus::DiscussionTime
        } else {
            GameStatus::NotStarted
        }
    }

    pub fn vote(&mut self, voter: &str, voted: &str) -> Result<(), &'static str> {
        let voter = self
            .players
            .iter()
            .position(|x| *x == voter)
            .ok_or("Player not found")?;
        let votee = self.players.iter().position(|x| *x == voted);
        self.votes[voter] = votee;
        Ok(())
    }

    fn wolf_won(&self) -> bool {
        let mut counts = vec![0; self.players.len()];
        for vote in self.votes.iter().flatten() {
            counts[*vote] += 1;
        }
        let max = counts.iter().max().unwrap();
        counts[self.wolf.unwrap()] == *max
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
