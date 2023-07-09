use crate::*;

// Include `NKI_DESCS`.
include!(concat!(env!("OUT_DIR"), "/nkis.rs"));

use std::collections::HashMap;

extern crate fastrand;
use once_cell::sync::Lazy as OnceCell;
use pancurses::colorpair::ColorPair;

// Non-kitten item.
pub struct Object {
    pub symbol: char,
    pub desc: String,
}

impl Object {
    pub fn new(desc: &str) -> Self {
        let symbols: OnceCell<Vec<char>> = OnceCell::new(|| {
            let first = 0_u8 as char;
            let last = 128_u8 as char;
            (first..last)
                .filter(|&c| c != ' ' && !c.is_ascii_control())
                .collect()
        });
        let symbol = *fastrand::choice(symbols.iter()).unwrap();
        let desc = desc.to_owned();
        Self { symbol, desc }
    }
}

pub struct Arena {
    pub field: Window,
    pub status: Window,
    pub lr: Coord,
    pub kitten_coord: Coord,
    pub objects: HashMap<Coord, Object>,
    pub foreground: u32,
}

#[macro_export]
macro_rules! show_status {
    ($arena:expr, $fmt:literal, $($args:expr),+) => {
        $arena.show_status(&format!($fmt, $($args),+));
    };
    ($arena:expr, $fmt:literal) => {
        $arena.show_status($fmt);
    };
}

impl Arena {
    pub fn new(top_window: Window) -> (Coord, Self) {
        // Set up windows.
        let top_lr = Coord::new(top_window.get_max_y(), top_window.get_max_x());
        let field_lr = Coord::new(
            (top_lr.row - 2).min(FIELD_MAX_ROWS),
            (top_lr.col).min(FIELD_MAX_COLS),
        );
        let field = top_window
            .subwin(field_lr.row + 2, field_lr.col + 2, 0, 0)
            .unwrap();
        field.keypad(true);
        let lr = Coord::new(field.get_max_y(), field.get_max_x());
        let status = top_window
            .subwin(1, top_lr.col, field_lr.row + 3, 0)
            .unwrap();
        status.keypad(true);

        // Shuffle up all the tiles.
        let mut coords: Vec<Coord> = (1..field_lr.row + 1)
            .flat_map(|r| (1..field_lr.col + 1).map(move |c| Coord::new(r, c)))
            .collect();
        fastrand::shuffle(&mut coords);

        // Shuffle up all the NKI descriptions.
        let mut nki_descs = NKI_DESCS.to_vec();
        fastrand::shuffle(&mut nki_descs);
        let nnkis = nki_descs.len().min(MAX_NKIS);

        // Set up the game.
        let player_coord = coords[0];
        let kitten_coord = coords[1];
        let mut objects = HashMap::with_capacity(nnkis + 2);
        let _ = objects.insert(kitten_coord, Object::new("your beautiful kitten!"));
        let coords = &coords[2..nnkis + 2];
        let nki_descs = &nki_descs[..nnkis];
        let more = coords
            .iter()
            .zip(nki_descs.iter())
            .map(|(&c, d)| (c, Object::new(d)));
        objects.extend(more);

        // Mess around with the background styling.
        let foreground = if has_colors() {
            start_color();
            use_default_colors();
            init_pair(1, COLOR_GREEN, -1);
            init_pair(2, -1, -1);
            let green = ColorPair(1);
            field.bkgd('.' as u32 | chtype::from(green));
            field.erase();
            chtype::from(ColorPair(2))
        } else {
            0
        };

        // Create the arena and return the info.
        let arena = Self {
            field,
            status,
            kitten_coord,
            objects,
            lr,
            foreground,
        };
        arena.render_objects();
        (player_coord, arena)
    }

    pub fn field_posn(&self) -> Coord {
        Coord::new(self.field.get_cur_y(), self.field.get_cur_x())
    }

    pub fn show_status(&self, msg: &str) {
        self.clear_status();
        let posn = self.field_posn();
        self.status.mvaddstr(0, 0, msg);
        self.field.mv(posn.row, posn.col);
    }
    pub fn clear_status(&self) {
        let posn = self.field_posn();
        self.status.mv(0, 0);
        self.status.clrtoeol();
        self.field.mv(posn.row, posn.col);
    }

    pub fn render(&self) {
        self.field.border('|', '|', '-', '-', '+', '+', '+', '+');
        self.status.refresh();
        self.field.refresh();
    }

    pub fn render_objects(&self) {
        for (coord, obj) in self.objects.iter() {
            self.field
                .mvaddch(coord.row, coord.col, obj.symbol as u32 | self.foreground);
        }
    }
}
