use crossterm::event::{KeyEvent, KeyCode};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};
use crate::events::AKEvent;

#[derive(Copy, Clone)]
pub enum PrimaryModifier {
    CtrlX,
    CtrlC,
}

pub struct Modifiers {
    pub primary_modifier: Option<PrimaryModifier>,
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
}

impl Modifiers {
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>) -> Self {
        Self {
            primary_modifier: None,
            queue,
        }
    }

    pub fn handle_modifier_key(&mut self, key: KeyEvent) {
        if let Some(_) = self.primary_modifier {
            self.handle_secondary_key(key);
        } else {
            self.handle_primary_key(key);
        }
    }
}

impl Modifiers {
    fn handle_secondary_key(&self, key: KeyEvent) {
        let pmodifier = self.primary_modifier.unwrap();
        match pmodifier {
            PrimaryModifier::CtrlX => self.handle_ctrl_x(key),
            PrimaryModifier::CtrlC => self.handle_ctrl_c(key),
        }
    }

    fn handle_primary_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    'x' => self.primary_modifier = Some(PrimaryModifier::CtrlX),
                    'c' => self.primary_modifier = Some(PrimaryModifier::CtrlC),
                    _ => panic!("Modifier notsupported {:?}", key ),
                }
            },
            _ => panic!("Primary modifier key not supported {:?}", key),
        }
    }

    fn handle_ctrl_x(&self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    'f' => AKEvent::send_event(Rc::clone(&self.queue)),
                    _ => panic!("CtrlX notsupported {:?}", key ),
                }
            }
            _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
        }
    }

    fn handle_ctrl_c(&self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    _ => panic!("CtrlC notsupported {:?}", key ),
                }
            }
            _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
        }
    }
}
