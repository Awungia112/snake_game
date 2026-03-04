// Game entities - obstacles, powerups, and cell types

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Powerup {
    pub x: i32,
    pub y: i32,
    pub kind: PowerupType,
    pub duration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerupType {
    Ghost,
    Multiplier,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    SnakeHead,
    SnakeBody,
    Food,
    Obstacle,
    PowerupGhost,
    PowerupMultiplier,
}

pub type ActiveEffects = HashMap<PowerupType, f64>;
