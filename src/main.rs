use serde::{Deserialize, Serialize};
use std::env;
use sudoku_generator::utils;

mod types;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Row {
    id: String,
    key: u128,
    value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QueryResult {
    total_rows: u128,
    offset: u128,
    rows: Vec<Row>,
}

fn make_puzzle(grid: &Vec<u8>, clue_count: u8, map_id: String) -> types::Puzzle {
    let mut puzzle: [u8; 81] = [0; 81];
    let idx_vals: Vec<u8> = (0..81).collect();

    let mut count = 0;

    while count < clue_count {
        let idx = utils::pick(&idx_vals).unwrap() as usize;

        if puzzle[idx] == 0 {
            puzzle[idx] = grid[idx];
            count += 1;
        }
    }
    types::Puzzle::new(map_id.as_str(), clue_count, &puzzle[..])
}

async fn total_rows() -> Result<u128, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:5984/sudoku/_design/puzzles/_view/completed?limit=1")
        .basic_auth("admin", Option::from("Bardop0nd"))
        .send()
        .await?
        .json::<QueryResult>()
        .await?;

    //println!("total docs in db: {}", response.total_rows);
    Ok(response.total_rows)
}

async fn get_solution(
    client: &reqwest::Client,
    pick: u128,
) -> Result<QueryResult, Box<dyn std::error::Error>> {
    let pick_str = pick.to_string();
    let q = vec![("limit", "1"), ("skip", pick_str.as_str())];
    let map = client
        .get("http://localhost:5984/sudoku/_design/puzzles/_view/completed")
        .basic_auth("admin", Option::from("Bardop0nd"))
        .query(q.as_slice())
        .send()
        .await?
        .json::<QueryResult>()
        .await?;

    Ok(map)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: spd <num_clues>");
        return Ok(());
    }

    let n = args[1].parse::<u8>().unwrap();

    let client = reqwest::Client::new();
    let total_rows = total_rows().await?;

    let range = (0..total_rows).collect::<Vec<u128>>();
    let pick = utils::pick(range.as_slice()).unwrap();

    let grid = get_solution(&client, pick - 1).await?;

    let r = grid.rows[0].clone();
    let puzzle = make_puzzle(&r.value, n, r.id);
    puzzle.dump_console();

    Ok(())
}
