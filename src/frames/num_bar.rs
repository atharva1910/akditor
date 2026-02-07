use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Block, Borders, BorderType},
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
        let mut text: String = String::new();
        for i in 1..= area.height  {
            text += format!("{}\n", i).as_str();
        }
        let para = Paragraph::new(text)
            .block(Block::new()
                  .borders(Borders::ALL)
                  .border_type(ratatui::widgets::BorderType::Rounded)
            );
        para.render(area, buf);
    }
    fn handle_key_event(&mut self, key: KeyEvent) {
        panic!("Key: {:?} not handled in status_bar", key);
    }

    fn quit(&self) -> bool {
        false
    }
}
