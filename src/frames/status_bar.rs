use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
};
use crossterm::event::KeyEvent;
use crate::{
    frames::frame_fn::FramesFn,
    frames::events::AKEvent,
    frames::cursor::{Cursor, CursorMove},
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub struct StatusBar {
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
    cursor: Cursor,
    area: Rect,
    display_str: Option<String>,
}

impl StatusBar {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>, area: Rect) -> Box<StatusBar> {
        Box::new(Self{
            queue,
            cursor: Cursor::new(area.width, area.height),
            area,
            display_str: None
        })
    }

    pub fn set_display_string(&mut self, display_str: Option<String>) {
        self.display_str = display_str;
    }
}

impl FramesFn for StatusBar {
    fn render(&self, area: Rect, buf: &mut Buffer)  {
        if let Some(s) = self.display_str.clone() {
            let para =
                Paragraph::new(s)
                .block(Block::new()
                       .borders(Borders::ALL));
            para.render(area, buf);
        } else {
            let blk = Block::new().borders(Borders::ALL);
            blk.render(area, buf);
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        panic!("Key: {:?} not handled in status_bar", key);
    }

    fn quit(&self) -> bool {
        false
    }

    fn get_cursor_pos(&self) -> Position {
        self.cursor.get_cursor_pos()
    }

    fn resize(&mut self, area: Rect) {
        self.area = area;
        self.cursor.resize(self.area.width, self.area.height);
    }
}
