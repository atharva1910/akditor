use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use crossterm::event::KeyEvent;
use crate::{
    frames:: FramesFn,
    events::AKEvent,    
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub struct StatusBar {
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
}

impl StatusBar {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>) -> StatusBar {
        Self{queue}
    }
}

impl FramesFn for StatusBar {
    fn render(&self, area: Rect, buf: &mut Buffer)  {
        let para = Paragraph::new("status_bar");
        para.render(area, buf);
    }
    fn handle_key_event(&mut self, key: KeyEvent) {
        panic!("Key: {:?} not handled in status_bar", key);
    }
}
