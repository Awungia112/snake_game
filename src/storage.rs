use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    pub high_score: u32,
    pub difficulty: Difficulty,
    pub theme_index: usize,
    pub obstacle_interval: f64,
    pub powerup_interval: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn speed(&self) -> f64 {
        match self {
            Difficulty::Easy => 0.25,
            Difficulty::Medium => 0.15,
            Difficulty::Hard => 0.10,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn next(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            high_score: 0,
            difficulty: Difficulty::Medium,
            theme_index: 0,
            obstacle_interval: 5.0,
            powerup_interval: 10.0,
        }
    }
}

impl GameData {
    fn get_save_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".snake_game_save.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::get_save_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
        GameData::default()
    }

    pub fn save(&self) {
        let path = Self::get_save_path();
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, json);
        }
    }
}
