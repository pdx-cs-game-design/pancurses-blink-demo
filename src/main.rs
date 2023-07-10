use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

pub use pancurses::*;

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
    assert_eq!(0, set_blink(true));
    let lr = Coord::new(window.get_max_y(), window.get_max_x());
    let center = Coord::new(lr.row / 2, lr.col / 2);
    let radius = lr.row.min(lr.col) as f32 / 2.0 - 2.0;
    let mut angle = 0.0f32;
    let mut posn = polar(center, radius, angle);
    let frame_time = Duration::from_millis(50);
    let mut blink = false;
    let robot = [
        '#' as chtype,
        '#' as chtype | chtype::from(Attribute::Blink),
    ];

    loop {
        angle = (angle + PI / 128.0) % (2.0 * PI);
        let new_posn = polar(center, radius, angle);

        if new_posn != posn {
            posn = new_posn;
            window.mvaddch(posn.row, posn.col, robot[blink as usize]);
            blink = !blink;
            window.mv(posn.row, posn.col);
            window.refresh();
        }
        sleep(frame_time);
    }
}
