use rand::seq::{IteratorRandom, SliceRandom};
use rand::thread_rng;

fn main() {
    //let topics: Vec<Vec<String>> = bincode::deserialize(include_bytes!("../topics.bin")).unwrap();
    let topics: Vec<Vec<String>> = serde_json::from_str(include_str!("../topics.json")).unwrap();
    let mut rng = thread_rng();
    let chosen = {
        let mut t = vec![];
        while t.len() != 2 {
            t = topics
                .choose(&mut rng)
                .expect("Empty topics")
                .iter()
                .filter(|x| x.len() < 18)
                .choose_multiple(&mut rng, 2);
        }
        t
    };
    println!("{:?}", chosen);
}
