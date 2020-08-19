use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Puzzle {
    n_clues: u8,
    grid: Vec<u8>,
    source_id: String,
    generated_millis: u128,
}

impl Puzzle {
    pub fn new(map_id: &str, n_clues: u8, puzzle_grid: &[u8]) -> Puzzle {
        let mut g: Vec<u8> = vec![0; 81];
        g.clone_from_slice(puzzle_grid);
        Puzzle {
            n_clues,
            source_id: String::from(map_id),
            grid: g,
            generated_millis: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }
    }

    pub fn dump_console(&self) {
        let as_json = serde_json::to_string(self).unwrap();
        println!("{}", as_json);
    }
}
