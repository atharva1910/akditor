use crate::{
    frames:: FramesFn,
    cursor::{CursorMove, Cursor},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};


const GAP_BUFFER_SIZE: usize = 1024;
const GAP_BUFFER_CHAR: char = ' ';

pub struct FileFrame {
    gap_start: usize,
    gap_end: usize,
    buffer: Vec<char>,
    cursor: Cursor,
    name: String,
}

impl FramesFn for FileFrame {
    fn handle_key_event(&mut self, key: KeyEvent) {
        // Handle key events specific to FileFrame state
        if key.is_press() {
            if key.modifiers == KeyModifiers::CONTROL {
        //        self.handle_ctrl_commands(key);
            } else {
                self.handle_key_event_pressed(key);
            }
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text = self.buffer[..self.gap_start].iter().collect::<String>()
            + &self.buffer[self.gap_end..].iter().collect::<String>();

        let para = Paragraph::new(text);

        para.render(area, buf);
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cursor.resize(cols, rows);
    }

    fn quit(&self) -> bool {
        false
    }
}

impl FileFrame {
    pub fn new(cols: u16, rows: u16) -> Box<FileFrame> {
        Box::new(Self {
            gap_start: 0,
            gap_end: GAP_BUFFER_SIZE - 1,
            buffer: vec![GAP_BUFFER_CHAR; GAP_BUFFER_SIZE],
            cursor: Cursor::new(cols, rows),
            name: "scratch".to_string(),
        })
    }

    fn handle_char_input(&mut self, c: char) {
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
        self.cursor.move_cursor(CursorMove::Right);
    }

    fn handle_key_event_pressed(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.handle_char_input(c);
            }

            KeyCode::Enter => {
                self.handle_char_input('\r');
                self.handle_char_input('\n');
                self.cursor.move_cursor(CursorMove::Down);
            }

            KeyCode::Backspace => {
                if self.cursor.move_cursor(CursorMove::Left) {
                    self.buffer[self.gap_start - 1] = GAP_BUFFER_CHAR;
                    self.gap_start -= 1;
                }
            }

            KeyCode::Left => {
                if self.cursor.move_cursor(CursorMove::Left) {
                    self.buffer[self.gap_end] = self.buffer[self.gap_start - 1];
                    self.gap_start -= 1;
                    self.gap_end -= 1;
                }
            }

            KeyCode::Right => {
                if self.cursor.move_cursor(CursorMove::Right) {
                    self.buffer[self.gap_start] = self.buffer[self.gap_end + 1];
                    self.gap_start += 1;
                    self.gap_end += 1;
                }
            }

            KeyCode::Tab => {
                for _ in 0..4 {
                    if self.cursor.move_cursor(CursorMove::Right) {
                        self.handle_char_input(' ');
                    }
                }
            }

            KeyCode::Up => if self.cursor.move_cursor(CursorMove::Up) {},

            KeyCode::Down => if self.cursor.move_cursor(CursorMove::Down) {},

            _ => {}
        }
    }
}
