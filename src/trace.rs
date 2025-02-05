use super::{save::SavedTrace, Pin, PinType, State};
use std::cell::RefCell;
use std::rc::Rc;

/// A Trace that connects two chip's Pin
#[derive(Default, Debug)]
pub struct Trace {
    link: Vec<Rc<RefCell<Pin>>>,
}

impl Trace {
    pub fn new() -> Self {
        Self { link: vec![] }
    }

    pub fn connect(&mut self, pin: Rc<RefCell<Pin>>) {
        self.link.push(pin);
    }

    pub fn communicate(&mut self) {
        let mut main_state = State::Undefined;
        for pin in &self.link {
            if pin.borrow().pin_type == PinType::Output {
                match pin.borrow().state {
                    State::High => main_state = State::High,
                    State::Low => {
                        if main_state == State::Undefined {
                            main_state = State::Low;
                        }
                    }
                    State::Undefined => {}
                }
            }
        }
        for pin in &mut self.link {
            if pin.borrow().pin_type != PinType::Output {
                pin.borrow_mut().state = main_state.clone();
            }
        }
    }

    pub fn save(&self) -> SavedTrace {
        let mut save = SavedTrace::new();
        for pin in &self.link {
            save.add_trace(pin.borrow().clone());
        }
        save
    }
}
