use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    //let topics: Vec<Vec<String>> = bincode::deserialize(include_bytes!("../topics.bin")).unwrap();
    let topics: Vec<Vec<String>> = serde_json::from_str(include_str!("../topics.json")).unwrap();
    let mut rng = thread_rng();
    println!(
        "{:?}",
        topics
            .choose(&mut rng)
            .expect("Empty topics")
            .choose_multiple(&mut rng, 2)
    );
}
