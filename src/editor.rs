use crate::{
    frames::{
        events::AKEvent,
        frame_fn::FramesFn,
        status_bar::StatusBar,
        num_bar::NumBar,
        file_frame::FileFrame,
        list_buffer::ListBuffer,
        file_explorer::FileExp,
    },
    modifiers::Modifiers,
};
use crossterm::event::{
    self,
    Event,
    KeyEvent,
};
use ratatui::{
    DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Direction, Layout, Position, Rect, Size}, widgets::Widget
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

enum CursorFrame {
    Status,
    File,
}
pub struct Editor {
    quit: bool,
    frame_stack: Vec<Box<dyn FramesFn>>,
    cur_frame: Option<usize>,
    status_bar: Box<StatusBar>,
    num_bar: Box<NumBar>,
    event_loop: Rc<RefCell<VecDeque<AKEvent>>>,
    modifier: Modifiers,
    cursor_frame: CursorFrame
}

impl Editor {
    //    pub fn new(size: ratatui::layout::Size, logger: &'a Logger) -> Self {
    pub fn new() -> Self {
        let queue =  Rc::new(RefCell::new(VecDeque::new()));
        queue.borrow_mut().push_back(AKEvent::NewBuffer);
        Editor {
            frame_stack: Vec::new(),
            cur_frame: None,
            modifier: Modifiers::new(Rc::clone(&queue)),
            status_bar: StatusBar::new(Rc::clone(&queue)),
            num_bar: NumBar::new(Rc::clone(&queue)),
            event_loop: queue,
            quit: false,
            cursor_frame: CursorFrame::File,
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) {
        while !self.quit {
            let _ = term.draw(|f| self.draw(f));

            if let Ok(event) = event::read() {
                self.handle_event(event);
            }

            self.update();
        }
    }

    fn update(&mut self) {
        if self.event_loop.borrow_mut().is_empty() {
            return;
        }

        let event = self.event_loop.borrow_mut().pop_front().unwrap();
        match event {
            AKEvent::NewBuffer => {
                let scratch = FileFrame::new(Rc::clone(&self.event_loop));
                self.push_frame(scratch, true);
            },
            AKEvent::FileExp => {
                let file_exp = FileExp::new(Rc::clone(&self.event_loop));
                self.push_frame(file_exp, true);
            },
            AKEvent::ListBuffer => {
                let mut frame_info: Vec<String> = Vec::new();
                for _ in self.frame_stack.iter() {
                    frame_info.push(String::from("test"));
                }
                let list_buf = ListBuffer::new(Rc::clone(&self.event_loop), frame_info);
                self.push_frame(list_buf, true);
            }
            AKEvent::Quit => {
                self.quit = true;
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let layout =
            Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(95), Constraint::Percentage(5)],
            );

        let [mode_area, status_area] = layout.areas(frame.area());

        //let layout =
        //    Layout::new(
        //        Direction::Horizontal,
        //        [Constraint::Fill(1), Constraint::Percentage(98)],
        //    );
        //let [num_area, mode_area] = layout.areas(mode_area);

        let mut x: u16 = 0;
        let mut y: u16 = 0;

        match self.cursor_frame {
            CursorFrame::Status => panic!("Cursor in status field not supported yet"),
            _ => {
                if let Some(i) = self.cur_frame {
                    (x, y) = self.frame_stack[i].get_cursor_pos();
                }
            }
        }
        frame.set_cursor_position(Position {
            x: mode_area.x + x,
            y: mode_area.y + y
        });
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
        //self.size.width = cols;
        //self.size.height = rows;
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if key.is_release() {
            return;
        }

        if self.modifier.is_modifier_key(key) {
            self.modifier.handle_modifier_key(key);
        } else if let Some(i) = self.cur_frame {
            self.frame_stack[i].handle_key_event(key);
        }
    }

    fn push_frame(&mut self, frame: Box<dyn FramesFn>, active: bool) {
        self.frame_stack.push(frame);
        if active {
            self.cur_frame = Some(self.frame_stack.len() - 1);
        }
    }
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout =
            Layout::new(
                Direction::Vertical,
                [Constraint::Percentage(95), Constraint::Percentage(5)],
        );

        let [mode_area, status_area] = layout.areas(area);

        //let layout =
        //    Layout::new(
        //        Direction::Horizontal,
        //        [Constraint::Fill(1), Constraint::Percentage(98)],
        //);
        //let [num_area, mode_area] = layout.areas(mode_area);

        if let Some(i) = self.cur_frame {
            self.frame_stack[i].render(mode_area, buf);
        }
        //self.num_bar.render(num_area, buf);
        self.status_bar.render(status_area, buf);
    }
}
