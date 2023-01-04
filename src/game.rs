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

pub static GAME_MANAGER: Lazy<std::sync::RwLock<GameManager>> =
    Lazy::new(|| std::sync::RwLock::new(GameManager {}));

pub struct GameManager {}
impl GameManager {}

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
