use crate::{
    file_frame::FileFrame,
    frames::FramesFn,
    ctrl::CtrlCmds,
    status_bar::StatusBar,
};
use ratatui::{
    buffer::Buffer,
    Frame, layout::{
        Rect, Constraint, Size, Layout, Direction,
    }, widgets::Widget
};
use crossterm::event::{self, KeyModifiers, KeyEvent, Event};

pub struct Editor {
    frame_stack: Vec<Box<dyn FramesFn>>,
    cur_frame: Option<usize>,
    prev_cmd: Option<CtrlCmds>,
    status_bar: StatusBar,
}

impl Editor {
//    pub fn new(size: ratatui::layout::Size, logger: &'a Logger) -> Self {
    pub fn new(size: Size) -> Self {
        let scratch = FileFrame::new(size.width, size.height);
        Editor {
            frame_stack: vec![scratch],
            cur_frame: Some(0),
            prev_cmd: None,
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

        if key.modifiers == KeyModifiers::CONTROL {
            panic!("Not handled ctrl modifier");
        } else if let Some(idx) = self.cur_frame {
            self.frame_stack[idx].handle_key_event(key);
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(95),
                Constraint::Percentage(5),
            ]);

        let [mode_area, status_area] = layout.areas(area);
        self.frame_stack[0].render(mode_area, buf);
        self.status_bar.render(status_area, buf);
    }
}
