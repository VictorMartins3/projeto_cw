use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::game::Game;

pub fn read_log_file(file_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn parse_log(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::new();
    let mut current_game = Game::new();
    let mut count_init = 0;

    for (_index, line) in lines.iter().enumerate() {
        if line.contains("InitGame") {
            if count_init != 0 {
                games.push(current_game);
                current_game = Game::new();
            }
            count_init += 1;
        } else if line.contains("ClientUserinfoChanged") {
            if let Some(name) = line.split("n\\").nth(1).and_then(|s| s.split('\\').next()) {
                current_game.add_player(name.to_string());
            }
        } else if line.contains("Kill") {
            if let Some(kill_info) = line.split(": ").nth(2) {
                let kill_parts: Vec<&str> = kill_info.split(' ').collect();
                if kill_parts.len() > 4 {
                    let means_of_death = *kill_parts.last().unwrap_or(&"");
                    current_game.add_kill(kill_parts[0], kill_parts[2], means_of_death);
                }
            }
        }
    }

    if current_game.total_kills > 0 {
        games.push(current_game);
    }

    games
}
