use serde::{Deserialize, Serialize};
use std::{env, thread, time};
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

    let url = format!(
        "http://{host}:{port}/sudoku/_design/puzzles/_view/completed?limit=1",
        host = db_host(),
        port = db_port()
    );

    let response = client
        .get(&url)
        .basic_auth("admin", Option::from(db_pw()))
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

    let url = format!(
        "http://{host}:{port}/sudoku/_design/puzzles/_view/completed",
        host = db_host(),
        port = db_port()
    );
    let map = client
        .get(&url)
        .basic_auth(db_admin(), Option::from(&db_pw()))
        .query(q.as_slice())
        .send()
        .await?
        .json::<QueryResult>()
        .await?;

    Ok(map)
}

fn db_host() -> String {
    env::var("DB_HOST").unwrap_or("10.0.1.108".to_string())
}

fn db_port() -> String {
    env::var("DB_PORT").unwrap_or("5984".to_string())
}

fn db_admin() -> String {
    env::var("DB_ADMIN_USER").unwrap_or("admin".to_string())
}

fn db_pw() -> String {
    env::var("DB_ADMIN_PW").unwrap_or("Bardop0nd".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {} <num_clues> -d", args[0]);
        return Ok(());
    }

    // TODO: fast hack for now. this should be a param or be read in from env
    let is_daemon: bool = args.len() == 3;

    let n = args[1].parse::<u8>().unwrap();

    let client = reqwest::Client::new();
    loop {
        let total_rows = total_rows().await?;

        let range = (0..total_rows).collect::<Vec<u128>>();
        let pick = utils::pick(range.as_slice()).unwrap();

        let grid = get_solution(&client, pick - 1).await?;

        let r = grid.rows[0].clone();
        let puzzle = make_puzzle(&r.value, n, r.id);
        puzzle.dump_console();

        if is_daemon == false {
            break;
        }
        //TODO: this should be retrieved from the environment
        thread::sleep(time::Duration::from_secs(5));
    }

    Ok(())
}
