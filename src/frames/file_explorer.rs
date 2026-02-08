use crate::{
    frames::frame_fn::FramesFn,
    cursor::{CursorMove, Cursor},
    frames::events::AKEvent,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget, Block, Borders, BorderType},
};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
    fs::read_dir,
    path::PathBuf,
};

const GAP_BUFFER_SIZE: usize = 1024;
const GAP_BUFFER_CHAR: char = ' ';

pub struct FileExp {
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
}

impl FileExp {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>) -> Box<FileExp> {
        Box::new(Self {
            queue
        })
    }
}

impl FramesFn for FileExp {
    fn handle_key_event(&mut self, key: KeyEvent) {
        self.handle_key_event_pressed(key);
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let f = read_dir(".").unwrap().map(|d| d.map(|e| e.path())).collect::<Result<Vec<PathBuf>, _>>().unwrap();
        let files: String =
            f .into_iter()
            .filter_map(|arg| arg.into_os_string().to_str().map(|s| format!("{}\n", s))).collect();

        let para =
            Paragraph::new(files)
            .block(Block::new()
                   .borders(Borders::ALL)
                   .border_type(BorderType::Rounded));
        para.render(area, buf);


    }

    fn quit(&self) -> bool {
        false
    }
}

impl FileExp {
    fn handle_key_event_pressed(&mut self, key: KeyEvent) {}
}
