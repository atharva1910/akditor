use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};
use crossterm::event::KeyEvent;
use crate::frames::FramesFn;

pub struct StatusBar {
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
