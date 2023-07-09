use crate::*;

pub struct Player<'a> {
    pub loc: Coord,
    pub arena: &'a Arena,
    pub clear_status: bool,
    pub kitten_found: bool,
}

impl<'a> Player<'a> {
    pub fn new(arena: &'a Arena, loc: Coord) -> Self {
        Self {
            loc,
            arena,
            clear_status: false,
            kitten_found: false,
        }
    }

    pub fn render(&self) {
        self.arena.field.mvaddch(
            self.loc.row,
            self.loc.col,
            '#' as u32 | self.arena.foreground,
        );
        self.arena.field.mv(self.loc.row, self.loc.col);
    }

    // XXX: The `move` keyword was a bad Rust idea.
    pub fn moove(&mut self, drow: i32, dcol: i32) {
        // Manage the status.
        if self.clear_status {
            self.arena.clear_status();
            self.clear_status = false;
        }

        // Draw what symbol is where we are after move.
        let obj = self.arena.objects.get(&self.loc);
        let ch = if let Some(obj) = obj {
            obj.symbol as u32
        } else {
            self.arena.field.getbkgd()
        };
        self.arena.field.mvaddch(self.loc.row, self.loc.col, ch);

        // Move.
        self.loc.row += drow;
        self.loc.col += dcol;

        // Show status if needed. Mark it to be cleared.
        let obj = self.arena.objects.get(&self.loc);
        if let Some(obj) = obj {
            show_status!(self.arena, "You see {}", obj.desc);
            self.clear_status = true;
            if self.arena.kitten_coord == self.loc {
                self.kitten_found = true;
            }
        }

        // Render ourself at new position.
        self.render();
    }

    pub fn get_command(&self) -> Command {
        loop {
            match self.arena.field.getch() {
                Some(Input::Character(ch)) => match ch {
                    'q' | 'Q' | '\u{1b}' => return Command::Quit,
                    'h' => return Command::Move(Dirn::Left),
                    'j' => return Command::Move(Dirn::Down),
                    'k' => return Command::Move(Dirn::Up),
                    'l' => return Command::Move(Dirn::Right),
                    _ => (),
                },
                Some(Input::KeyLeft) => return Command::Move(Dirn::Left),
                Some(Input::KeyRight) => return Command::Move(Dirn::Right),
                Some(Input::KeyUp) => return Command::Move(Dirn::Up),
                Some(Input::KeyDown) => return Command::Move(Dirn::Down),
                None => panic!("getch returned None"),
                _ => (),
            }
        }
    }
}
