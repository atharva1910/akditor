use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
};

pub trait FramesFn {
    //fn new() -> Box<dyn FramesFn>;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_key_event(&mut self, key: KeyEvent);

    fn resize(&mut self, cols: u16, rows: u16) {
    }
    fn quit(&self) -> bool {
        false
    }
}
