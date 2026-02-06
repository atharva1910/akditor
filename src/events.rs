use std::{
    collections::VecDeque,
    cell::RefCell,
    rc::Rc,
};

pub enum AKEvent {
    NewBuffer,
}

impl AKEvent {
    pub fn send_event(queue: Rc<RefCell<VecDeque<AKEvent>>>) {
        queue.borrow_mut().push_back(AKEvent::NewBuffer);
    }
}
