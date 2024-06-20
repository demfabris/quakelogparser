#![allow(unused_variables, dead_code)]
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

mod death_means;

use death_means::MeansOfDeath;

#[derive(Default, Debug)]
struct Game {
    total_kills: u32,
    players: Vec<String>,
    kills: HashMap<String, i32>,
    kills_by_means: HashMap<MeansOfDeath, u32>,
}

impl std::fmt::Display for Game {
    // pretty print game stats
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "  total_kills: {}", self.total_kills)?;
        writeln!(f, "  players: {:?}", self.players)?;
        writeln!(f, "  kills: {:?}", self.kills)?;
        writeln!(f, "  kills_by_means: {:?}", self.kills_by_means)
    }
}

impl Game {
    fn new() -> Self {
        Game::default()
    }

    fn add_player(&mut self, player: &str) {
        if !self.players.contains(&player.to_string()) {
            self.players.push(player.to_string());
        }
    }

    fn add_kill(&mut self, killer: &str, killed: &str, means: MeansOfDeath) {
        self.total_kills += 1;

        if killer != "<world>" {
            let killer_entry = self.kills.entry(killer.to_string()).or_insert(0);
            *killer_entry += 1;
            self.add_player(killer);
        } else {
            let killed_entry = self.kills.entry(killed.to_string()).or_insert(0);
            *killed_entry -= 1;
        }

        self.add_player(killed);

        let means_entry = self.kills_by_means.entry(means).or_insert(0);
        *means_entry += 1;
    }
}

fn parse_kill(line: &str) -> (String, String, MeansOfDeath) {
    let re = Regex::new(r"(\d+:\d+) Kill: (\d+) (\d+) (\d+): (.+) killed (.+) by (.+)").unwrap();
    let caps = re.captures(line).unwrap();
    let killer = caps.get(5).map_or("", |m| m.as_str());
    let killed = caps.get(6).map_or("", |m| m.as_str());
    let means = caps.get(7).map_or("", |m| m.as_str());
    (
        killer.to_string(),
        killed.to_string(),
        MeansOfDeath::from_str(means).unwrap_or_default(),
    )
}

fn parse_log(file_path: &Path) -> HashMap<String, Game> {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut games: HashMap<String, Game> = HashMap::new();
    let mut current_game: Option<Game> = None;
    let mut game_id = 1;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.contains("InitGame") {
            if let Some(game) = current_game.take() {
                games.insert(format!("game_{}", game_id), game);
                game_id += 1;
            }
            current_game = Some(Game::new());
        } else if line.contains("Kill:") {
            if let Some(game) = &mut current_game {
                let (killer, killed, means) = parse_kill(&line);
                game.add_kill(&killer, &killed, means);
            }
        }
    }

    if let Some(game) = current_game {
        games.insert(format!("game_{}", game_id), game);
    }

    games
}

fn print_report(games: &HashMap<String, Game>) {
    for (game_id, game) in games {
        println!("{}:", game_id);
        println!("{}", game);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{io::Read, path::PathBuf};

    #[test]
    fn run_full_sample_log() {
        let file_path = PathBuf::from("./test_assets/sample.log");
        let games = parse_log(&file_path);
        print_report(&games);
    }

    #[test]
    fn test_partial_sample_log() {
        let file_path = PathBuf::from("./test_assets/partial_sample.log");
        let mut log_data = String::new();
        File::open(&file_path)
            .expect("Unable to open file")
            .read_to_string(&mut log_data)
            .unwrap();
        let kill_event_count = log_data
            .lines()
            .filter(|line| line.contains("Kill:"))
            .count();
        let games = parse_log(&file_path);

        // assert kill count matches
        let total_kills: u32 = games.values().map(|game| game.total_kills).sum();
        assert_eq!(kill_event_count, total_kills as usize);
    }
}
