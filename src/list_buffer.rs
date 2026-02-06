use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};
use crate::{
    frames::FramesFn,
    events::AKEvent,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Block, Borders},
};
use crossterm::event::KeyEvent;

pub struct ListBuffer{
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
    frame_info: Vec<String>,
}

impl ListBuffer {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>, frame_info: Vec<String>) -> Box<ListBuffer> {
        Box::new(Self{queue, frame_info})
    }
}

impl FramesFn for ListBuffer {
    fn handle_key_event(&mut self, key: KeyEvent) {
        panic!("list_buf key event not handled {:?}", key);
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut text = String::new();
        for (i,s) in self.frame_info.iter().enumerate() {
            let frame_str = format!("{}: {}\n", i, s);
            text += frame_str.as_str();
        }
        let para = Paragraph::new(text).block(Block::new().borders(Borders::ALL));
        para.render(area, buf);
    }
}
