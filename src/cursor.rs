pub enum CursorMove {
    Up,
    Down,
    Left,
    Right,
}

pub struct Cursor {
    x: u16,
    y: u16,
    cols: u16,
    rows: u16,
}

impl Cursor {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            x: 0,
            y: 0,
            cols,
            rows,
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;
    }

    pub fn move_cursor(&mut self, dir: CursorMove) -> bool {
        match dir {
            CursorMove::Up => {
                if self.x == 0 {
                    return false;
                }
                self.x -= 1;
            }

            CursorMove::Down => {
                self.y += 1;
            }

            CursorMove::Left => {
                if self.x == 0 && self.y == 0 {
                    return false;
                }

                if self.x == 0 {
                    self.y -= 1;
                } else {
                    self.x -= 1;
                }
            }

            CursorMove::Right => {
                if self.x >= self.cols - 1 {
                    self.x = 0;
                    self.y += 1;
                } else {
                    self.x += 1
                }
            }
        }

        true
    }

    pub fn show_cursor(&self, frame: &mut ratatui::Frame) {
        frame.set_cursor_position(
            ratatui::layout::Position {
                x: self.x,
                y: self.y
            });
    }
}
