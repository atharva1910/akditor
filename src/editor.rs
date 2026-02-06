use crate::{
    modifiers::Modifiers,
    file_frame::FileFrame,
    frames::FramesFn,
    status_bar::StatusBar,
    events::AKEvent,
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
    status_bar: StatusBar,
    event_loop: Rc<RefCell<VecDeque<AKEvent>>>,
    modifier: Modifiers,
    cols: u16,
    rows: u16,
    pub quit: bool,
}

impl Editor {
    //    pub fn new(size: ratatui::layout::Size, logger: &'a Logger) -> Self {
    pub fn new(size: Size) -> Self {
        let queue =  Rc::new(RefCell::new(VecDeque::new()));
        let scratch = FileFrame::new(Rc::clone(&queue),size.width, size.height);

        Editor {
            frame_stack: vec![scratch],
            cur_frame: Some(0),
            modifier: Modifiers::new(Rc::clone(&queue)),
            status_bar: StatusBar::new(Rc::clone(&queue)),
            event_loop: queue,
            cols: size.width,
            rows: size.height,
            quit: false,
        }
    }

    pub fn update(&mut self) {
        if self.event_loop.borrow_mut().is_empty() {
            return;
        }

        let _ = self.event_loop.borrow_mut().pop_front().unwrap();
        let scratch = FileFrame::new(Rc::clone(&self.event_loop),self.cols, self.rows);
        self.frame_stack.push(scratch);
        self.cur_frame = Some(self.frame_stack.len()-1);
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            event::Event::Key(key) => self.handle_key_event(key),
            event::Event::Resize(cols, rows) => self.resize(cols, rows),
            _ => panic!("Not implemented event {:?}", event),
        }
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.cols = cols;
        self.rows = rows;

        // ToDO : Update all frames
        if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].resize(cols, rows);
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.is_release() {
            return;
        }

        if self.modifier.is_modifier_key(key) {
            self.modifier.handle_modifier_key(key);
        } else if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].handle_key_event(key);
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Vertical,
            //[Constraint::Percentage(95), Constraint::Percentage(5)],
            [Constraint::Percentage(90), Constraint::Percentage(10)],
        );

        let [mode_area, status_area] = layout.areas(area);
        if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].render(mode_area, buf);
        }
        self.status_bar.render(status_area, buf);
    }
}
