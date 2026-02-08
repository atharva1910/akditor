use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};
use crate::frames::events::AKEvent;

#[derive(Copy, Clone)]
pub enum PrimaryModifier {
    CtrlX,
    CtrlC,
}

pub struct Modifiers {
    mod_stack: Vec<PrimaryModifier>,
    queue: Rc<RefCell<VecDeque<AKEvent>>>,
}

impl Modifiers {
    pub fn is_modifier_key(&self, key: KeyEvent) -> bool {
        if key.modifiers == KeyModifiers::CONTROL ||
            self.mod_stack.len() != 0 {
            return true;
        }
        false
    }
    pub fn new(queue: Rc<RefCell<VecDeque<AKEvent>>>) -> Self {
        Self {
            mod_stack: Vec::new(),
            queue,
        }
    }

    pub fn handle_modifier_key(&mut self, key: KeyEvent) {
        // Todo fix this when multiple modifers are used
        if self.mod_stack.len() != 0 {
            self.handle_secondary_key(key);
        } else {
            self.handle_primary_key(key);
        }
    }
}

impl Modifiers {
    fn handle_secondary_key(&mut self, key: KeyEvent) {
        let pmodifier = self.mod_stack.pop().unwrap();
        match pmodifier {
            PrimaryModifier::CtrlX => self.handle_ctrl_x(key),
            PrimaryModifier::CtrlC => self.handle_ctrl_c(key),
        }
    }

    fn handle_primary_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    'x' => self.mod_stack.push(PrimaryModifier::CtrlX),
                    'c' => self.mod_stack.push(PrimaryModifier::CtrlC),
                    _ => panic!("Modifier notsupported {:?}", key ),
                }
            },
            _ => panic!("Primary modifier key not supported {:?}", key),
        }
    }

    fn handle_ctrl_x(&mut self, key: KeyEvent) {
        if key.modifiers == KeyModifiers::CONTROL {
            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'c' => self.queue.borrow_mut().push_back(AKEvent::Quit),
                        _ => panic!("CtrlX notsupported {:?}", key ),
                    }

                    self.mod_stack.clear();
                }
                _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
            }
        } else {
            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'f' => self.queue.borrow_mut().push_back(AKEvent::FileExp),
                        'b' => self.queue.borrow_mut().push_back(AKEvent::ListBuffer),

                        _ => panic!("CtrlX notsupported {:?}", key ),
                    }

                    self.mod_stack.clear();
                }
                _=> panic!("Only chars allowed for ctrl cmds: {:?}", key)
            }
        }
    }

    fn handle_ctrl_c(&mut self, key: KeyEvent) {
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
