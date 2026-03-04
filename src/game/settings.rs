// Game settings - spawn intervals and difficulty

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub obstacle_interval: f64,
    pub powerup_interval: f64,
    pub powerup_chance: f64,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            obstacle_interval: 5.0,
            powerup_interval: 10.0,
            powerup_chance: 0.2,
        }
    }
}
