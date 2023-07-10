use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

pub use pancurses::*;
//use pancurses::colorpair::ColorPair;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

fn polar(center: Coord, radius: f32, angle: f32) -> Coord {
    let row = center.row - (radius * angle.sin()).floor() as i32;
    let col = center.col - (radius * angle.cos()).floor() as i32;
    Coord::new(row, col)
}

fn main() {
    let window = initscr();
    let lr = Coord::new(window.get_max_y(), window.get_max_x());
    let center = Coord::new(lr.row / 2, lr.col / 2);
    let radius = lr.row.min(lr.col) as f32 / 2.0 - 2.0;
    let mut angle = 0.0f32;
    let mut posn = polar(center, radius, angle);
    let frame_time = Duration::from_millis(50);

    loop {
        angle = (angle + PI / 128.0) % (2.0 * PI);
        let new_posn = polar(center, radius, angle);

        if new_posn != posn {
            window.mvaddch(posn.row, posn.col, ' ');
            posn = new_posn;
            window.mvaddch(posn.row, posn.col, '#');
            window.mv(posn.row, posn.col);
            window.refresh();
        }
        sleep(frame_time);
    }
}
