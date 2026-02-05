use crate::{
    ctrl::Modifiers,
    file_frame::FileFrame,
    frames::FramesFn,
    status_bar::StatusBar
};
use crossterm::event::{
    self,
    Event,
    KeyEvent,
    KeyModifiers
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect, Size},
    widgets::Widget,
    Frame,
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub struct Editor {
    frame_stack: Vec<Box<dyn FramesFn>>,
    cur_frame: Option<usize>,
    modifier: Option<Modifiers>,
    status_bar: StatusBar,
    event_loop: Rc<RefCell<VecDeque<i32>>>,
}

impl Editor {
    //    pub fn new(size: ratatui::layout::Size, logger: &'a Logger) -> Self {
    pub fn new(size: Size) -> Self {
        let scratch = FileFrame::new(size.width, size.height);
        Editor {
            frame_stack: vec![scratch],
            cur_frame: Some(0),
            modifier: None,
            status_bar: StatusBar {},
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn quit(&self) -> bool {
        self.cur_frame.is_none()
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            event::Event::Key(key) => self.handle_key_event(key),
            event::Event::Resize(cols, rows) => self.resize(cols, rows),
            _ => panic!("Not implemented event {:?}", event),
        }
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].resize(cols, rows);
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.is_release() {
            return;
        }

        if (key.modifiers == KeyModifiers::CONTROL &&
            self.modifier.is_none()) ||
            self.modifier.is_some() {
            self.handle_ctrl_event(key);
        } else if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].handle_key_event(key);
        }
    }

    fn handle_modifier_fn(&mut self, key: KeyEvent) {
        if let Some(modifier) = self.modifier {
            match key.code {
                event::KeyCode::Char(_) => modifier.handle_modifier_fn(key),
                _ => panic!("Modifier fn key {:?} not supported", key),
            }
        }
    }

    fn handle_modifier(&mut self, key: KeyEvent) {
        match key.code {
            event::KeyCode::Char(c) => {
                match c {
                    'x' => self.modifier = Some(Modifiers::CTRL_X),
                    'c' => self.modifier = Some(Modifiers::CTRL_C),
                    _ => panic!("Modifier notsupported {:?}", key ),
                }
            }
            _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
        }
    }

    fn handle_ctrl_event(&mut self, key: KeyEvent) {
        if self.modifier.is_some() {
            self.handle_modifier_fn(key);
        } else {
            self.handle_modifier(key);
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(95), Constraint::Percentage(5)],
        );

        let [mode_area, status_area] = layout.areas(area);
        if let Some(idx) = self.cur_frame {
            self.frame_stack[0].render(mode_area, buf);
        }
        self.status_bar.render(status_area, buf);
    }
}
