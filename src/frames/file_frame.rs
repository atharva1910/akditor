use crate::{
    frames::frame_fn::FramesFn,
    frames::events::AKEvent,
    frames::cursor::{Cursor, CursorMove},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

const GAP_BUFFER_SIZE: usize = 1024;
const GAP_BUFFER_CHAR: char = ' ';

pub struct FileFrame {
    gap_start: usize,
    gap_end: usize,
    buffer: Vec<char>,
    name: String,
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
    cursor: Cursor,
    area: Rect,
}

impl FileFrame {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>, area: Rect) -> Box<FileFrame> {
        Box::new(Self {
            gap_start: 0,
            gap_end: GAP_BUFFER_SIZE - 1,
            buffer: vec![GAP_BUFFER_CHAR; GAP_BUFFER_SIZE],
            name: "scratch".to_string(),
            cursor: Cursor::new(area.width, area.height),
            area,
            queue
        })
    }
}

impl FramesFn for FileFrame {
    fn get_cursor_pos(&self) -> (u16, u16) {
         (0,0)
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        self.handle_key_event_pressed(key);
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let text =
            self.buffer[..self.gap_start].iter().collect::<String>()
            + &self.buffer[self.gap_end..].iter().collect::<String>();
        let para =
            Paragraph::new(text).wrap(Wrap{
                trim: true
            });
        para.render(area, buf);
    }

    fn quit(&self) -> bool {
        false
    }
}

impl FileFrame {
    fn handle_char_input(&mut self, c: char) {
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
        //self.cursor.move_cursor(CursorMove::Right);
    }

    fn handle_key_event_pressed(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.handle_char_input(c);
            }

            KeyCode::Enter => {
                self.handle_char_input('\r');
                self.handle_char_input('\n');
                //self.cursor.move_cursor(CursorMove::Down);
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
