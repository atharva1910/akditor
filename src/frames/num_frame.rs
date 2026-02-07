use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Block, Borders},
};
use crossterm::event::KeyEvent;
use crate::{
    frames::frame_fn::FramesFn,
    frames::events::AKEvent,
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub struct NumBar {
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
}

impl NumBar {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>) -> Box<NumBar> {
         Box::new(Self{queue})
    }
}

impl FramesFn for NumBar {
    fn render(&self, area: Rect, buf: &mut Buffer)  {
        let para = Paragraph::new("status_bar").block(Block::new().borders(Borders::ALL));
        para.render(area, buf);
    }
    fn handle_key_event(&mut self, key: KeyEvent) {
        panic!("Key: {:?} not handled in status_bar", key);
    }

    fn quit(&self) -> bool {
        false
    }
}
