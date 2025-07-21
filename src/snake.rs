use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw;

#[derive(Clone, Copy, PartialEq)]

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let head_color: Color = [0.0, 0.6, 0.0, 1.0];
        let eye_color: Color = [1.0, 1.0, 1.0, 1.0];
        let mut iter = self.body.iter();
        if let Some(head) = iter.next() {
            // Draw head as a rounded rectangle (ellipse for more distinction)
            let block_size = 25.0;
            let base_x = draw::to_coord(head.x);
            let base_y = draw::to_coord(head.y);
            let head_rect = [base_x, base_y, block_size, block_size];
            piston_window::ellipse(head_color, head_rect, con.transform, g);
            // Draw eyes based on direction
            let (eye1, eye2) = match self.direction {
                Direction::Up => (
                    [base_x + block_size * 0.25, base_y + block_size * 0.15, block_size * 0.15, block_size * 0.15],
                    [base_x + block_size * 0.60, base_y + block_size * 0.15, block_size * 0.15, block_size * 0.15],
                ),
                Direction::Down => (
                    [base_x + block_size * 0.25, base_y + block_size * 0.70, block_size * 0.15, block_size * 0.15],
                    [base_x + block_size * 0.60, base_y + block_size * 0.70, block_size * 0.15, block_size * 0.15],
                ),
                Direction::Left => (
                    [base_x + block_size * 0.10, base_y + block_size * 0.25, block_size * 0.15, block_size * 0.15],
                    [base_x + block_size * 0.10, base_y + block_size * 0.60, block_size * 0.15, block_size * 0.15],
                ),
                Direction::Right => (
                    [base_x + block_size * 0.75, base_y + block_size * 0.25, block_size * 0.15, block_size * 0.15],
                    [base_x + block_size * 0.75, base_y + block_size * 0.60, block_size * 0.15, block_size * 0.15],
                ),
            };
            piston_window::ellipse(eye_color, eye1, con.transform, g);
            piston_window::ellipse(eye_color, eye2, con.transform, g);
        }
        // Alternate body colors
        let mut is_dark = false;
        for block in iter {
            let color = if is_dark {
                [0.0, 0.8, 0.0, 1.0]
            } else {
                [0.0, 1.0, 0.0, 1.0]
            };
            draw::draw_block(color, block.x, block.y, con, g);
            is_dark = !is_dark;
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y - 1,
            },

            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let moving_dir = dir.unwrap_or(self.direction);
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            if ch == self.body.len() + 1 {
                break;
            }
        }
        false
    }
}
