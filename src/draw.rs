use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d, ellipse, Line};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_circle(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let radius = BLOCK_SIZE * 0.5;
    ellipse(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_grid(color: Color, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let line = Line::new(color, 0.5);
    for i in 0..=width {
        let x = to_coord(i);
        let y1 = to_coord(0);
        let y2 = to_coord(height);
        line.draw([
            x, y1,
            x, y2
        ], &con.draw_state, con.transform, g);
    }
    for j in 0..=height {
        let y = to_coord(j);
        let x1 = to_coord(0);
        let x2 = to_coord(width);
        line.draw([
            x1, y,
            x2, y
        ], &con.draw_state, con.transform, g);
    }
}
