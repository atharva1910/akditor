use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
};

pub trait FramesFn {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_key_event(&mut self, key: KeyEvent);
    fn quit(&self) -> bool;
}
