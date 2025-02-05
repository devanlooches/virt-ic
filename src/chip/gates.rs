//! Logic Gates like OR, AND, NOT ...
use super::{Chip, ChipInfo, Pin, PinType};
use crate::State;
use std::cell::RefCell;
use std::rc::Rc;

/// # A chip with 4 bundled "OR" gates
///
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///    B --|2   13|-- E
///  A|B --|3   12|-- F
///    C --|4   11|-- E|F
///    D --|5   10|-- G
///  C|D --|6    9|-- H
///  GND --|7    8|-- G|H
///        --------
/// ```
#[derive(Debug)]
pub struct GateOr {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateOr {
    fn default() -> Self {
        Self::new()
    }
}

impl GateOr {
    pub const TYPE: &'static str = "virt_ic::GateOr";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const A_OR_B: u8 = 3;
    pub const C: u8 = 4;
    pub const D: u8 = 5;
    pub const C_OR_D: u8 = 6;
    pub const E: u8 = 13;
    pub const F: u8 = 12;
    pub const E_OR_F: u8 = 11;
    pub const G: u8 = 10;
    pub const H: u8 = 9;
    pub const G_OR_H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for GateOr {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate OR",
            description: "A 4-in-one OR gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B
            self.pin[2].borrow_mut().state = if self.pin[0].borrow().state == State::High
                || self.pin[1].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // C && D
            self.pin[5].borrow_mut().state = if self.pin[3].borrow().state == State::High
                || self.pin[4].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // E && F
            self.pin[10].borrow_mut().state = if self.pin[11].borrow().state == State::High
                || self.pin[12].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // G && H
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High
                || self.pin[9].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Low;
            }
        }
    }
}

/// # A chip with 4 bundled "AND" gates
///
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///    B --|2   13|-- E
///  A&B --|3   12|-- F
///    C --|4   11|-- E&F
///    D --|5   10|-- G
///  C&D --|6    9|-- H
///  GND --|7    8|-- G&H
///        --------
/// ```
#[derive(Debug)]
pub struct GateAnd {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateAnd {
    fn default() -> Self {
        Self::new()
    }
}

impl GateAnd {
    pub const TYPE: &'static str = "virt_ic::GateAnd";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const A_AND_B: u8 = 3;
    pub const C: u8 = 4;
    pub const D: u8 = 5;
    pub const C_AND_D: u8 = 6;
    pub const E: u8 = 13;
    pub const F: u8 = 12;
    pub const E_AND_F: u8 = 11;
    pub const G: u8 = 10;
    pub const H: u8 = 9;
    pub const G_AND_H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for GateAnd {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate AND",
            description: "A 4-in-one AND gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B
            self.pin[2].borrow_mut().state = if self.pin[0].borrow().state == State::High
                && self.pin[1].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // C && D
            self.pin[5].borrow_mut().state = if self.pin[3].borrow().state == State::High
                && self.pin[4].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // E && F
            self.pin[10].borrow_mut().state = if self.pin[11].borrow().state == State::High
                && self.pin[12].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // G && H
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High
                && self.pin[9].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }
}

/// # A chip with 3 bundled "3-Input AND" gates
///
/// # Diagram
/// ```
///            ---__---
///        A --|1   14|-- VCC
///        B --|2   13|-- C
///        D --|3   12|-- A&B&C
///        E --|4   11|-- G
///        F --|5   10|-- H
///    D&E&F --|6    9|-- I
///      GND --|7    8|-- G&H&I
///            --------
/// ```
#[derive(Debug)]
pub struct Gate3InputAnd {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for Gate3InputAnd {
    fn default() -> Self {
        Self::new()
    }
}

impl Gate3InputAnd {
    pub const TYPE: &'static str = "virt_ic::Gate3InputAnd";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const C: u8 = 13;
    pub const A_AND_B_AND_C: u8 = 12;
    pub const D: u8 = 3;
    pub const E: u8 = 4;
    pub const F: u8 = 5;
    pub const D_AND_E_AND_F: u8 = 6;
    pub const G: u8 = 11;
    pub const H: u8 = 10;
    pub const I: u8 = 9;
    pub const G_AND_H_AND_I: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for Gate3InputAnd {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate 3-Input AND",
            description: "A 3-in-one 3-Input AND gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B && C
            self.pin[11].borrow_mut().state = if self.pin[0].borrow().state == State::High
                && self.pin[1].borrow().state == State::High
                && self.pin[12].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // D && E && F
            self.pin[5].borrow_mut().state = if self.pin[2].borrow().state == State::High
                && self.pin[3].borrow().state == State::High
                && self.pin[4].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
            // G && H && I
            self.pin[7].borrow_mut().state = if self.pin[10].borrow().state == State::High
                && self.pin[9].borrow().state == State::High
                && self.pin[8].borrow().state == State::High
            {
                State::High
            } else {
                State::Low
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }
}

/// # A chip with 6 bundled "NOT" gates
///
/// # Diagram
/// ```
///        ---__---
///    A --|1   14|-- VCC
///   !A --|2   13|-- D
///    B --|3   12|-- !D
///   !B --|4   11|-- E
///    C --|5   10|-- !E
///   !C --|6    9|-- F
///  GND --|7    8|-- !F
///        --------
/// ```
#[derive(Debug)]
pub struct GateNot {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateNot {
    fn default() -> Self {
        Self::new()
    }
}

impl GateNot {
    pub const TYPE: &'static str = "virt_ic::GateNot";

    pub const A: u8 = 1;
    pub const NOT_A: u8 = 2;
    pub const B: u8 = 3;
    pub const NOT_B: u8 = 4;
    pub const C: u8 = 5;
    pub const NOT_C: u8 = 6;
    pub const D: u8 = 13;
    pub const NOT_D: u8 = 12;
    pub const E: u8 = 11;
    pub const NOT_E: u8 = 10;
    pub const F: u8 = 9;
    pub const NOT_F: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for GateNot {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate NOT",
            description: "A 6-in-one NOT gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // !A
            self.pin[1].borrow_mut().state = if self.pin[0].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
            // !B
            self.pin[3].borrow_mut().state = if self.pin[2].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
            // !C
            self.pin[5].borrow_mut().state = if self.pin[4].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
            // !D
            self.pin[11].borrow_mut().state = if self.pin[12].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
            // !E
            self.pin[9].borrow_mut().state = if self.pin[10].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
            // !F
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High {
                State::Low
            } else {
                State::High
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }
}

/// # A chip with 4 bundled "NOR" gates
///
/// # Diagram
/// ```
///          ---__---
/// !(A|B) --|1   14|-- VCC
///      A --|2   13|-- !(E|F)
///      B --|3   12|-- E
/// !(C|D) --|4   11|-- F
///      C --|5   10|-- !(G|H)
///      D --|6    9|-- G
///    GND --|7    8|-- H
///          --------
/// ```
#[derive(Debug)]
pub struct GateNor {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateNor {
    fn default() -> Self {
        Self::new()
    }
}

impl GateNor {
    pub const TYPE: &'static str = "virt_ic::GateNor";

    pub const NOT_A_OR_B: u8 = 1;
    pub const A: u8 = 2;
    pub const B: u8 = 3;
    pub const NOT_C_OR_D: u8 = 4;
    pub const C: u8 = 5;
    pub const D: u8 = 6;
    pub const NOT_E_OR_F: u8 = 13;
    pub const E: u8 = 12;
    pub const F: u8 = 11;
    pub const NOT_G_OR_H: u8 = 10;
    pub const G: u8 = 9;
    pub const H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}

impl Chip for GateNor {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate NOR",
            description: "A 4-in-one NOR gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B
            self.pin[0].borrow_mut().state = if self.pin[1].borrow().state == State::High
                || self.pin[2].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // C && D
            self.pin[3].borrow_mut().state = if self.pin[4].borrow().state == State::High
                || self.pin[5].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // E && F
            self.pin[9].borrow_mut().state = if self.pin[8].borrow().state == State::High
                || self.pin[7].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // G && H
            self.pin[12].borrow_mut().state = if self.pin[11].borrow().state == State::High
                || self.pin[10].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Low;
            }
        }
    }
}

/// # A chip with 3 bundled "3-Input NOR" gates
///
/// # Diagram
/// ```
///            ---__---
///        A --|1   14|-- VCC
///        B --|2   13|-- C
///        D --|3   12|-- !(A|B|C)
///        E --|4   11|-- G
///        F --|5   10|-- H
/// !(D|E|F) --|6    9|-- I
///      GND --|7    8|-- !(G|H|I)
///            --------
/// ```
#[derive(Debug)]
pub struct Gate3InputNor {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for Gate3InputNor {
    fn default() -> Self {
        Self::new()
    }
}

impl Gate3InputNor {
    pub const TYPE: &'static str = "virt_ic::Gate3InputNor";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const C: u8 = 13;
    pub const NOT_A_OR_B_OR_C: u8 = 12;
    pub const D: u8 = 3;
    pub const E: u8 = 4;
    pub const F: u8 = 5;
    pub const NOT_D_OR_E_OR_F: u8 = 6;
    pub const G: u8 = 11;
    pub const H: u8 = 10;
    pub const I: u8 = 9;
    pub const NOT_G_OR_H_OR_I: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}

impl Chip for Gate3InputNor {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate 3-Input NOR",
            description: "A 3-in-one 3-Input NOR gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B && C
            self.pin[11].borrow_mut().state = if self.pin[0].borrow().state == State::Low
                && self.pin[1].borrow().state == State::Low
                && self.pin[12].borrow().state == State::Low
            {
                State::High
            } else {
                State::Low
            };
            // D && E && F
            self.pin[5].borrow_mut().state = if self.pin[2].borrow().state == State::Low
                && self.pin[3].borrow().state == State::Low
                && self.pin[4].borrow().state == State::Low
            {
                State::High
            } else {
                State::Low
            };
            // G && H && I
            self.pin[7].borrow_mut().state = if self.pin[10].borrow().state == State::Low
                && self.pin[9].borrow().state == State::Low
                && self.pin[8].borrow().state == State::Low
            {
                State::High
            } else {
                State::Low
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Low;
            }
        }
    }
}

/// # A chip with 4 bundled "NAND" gates
///
/// # Diagram
/// ```
///          ---__---
///      A --|1   14|-- VCC
///      B --|2   13|-- E
/// !(A&B) --|3   12|-- F
///      C --|4   11|-- !(E&F)
///      D --|5   10|-- G
/// !(C&D) --|6    9|-- H
///    GND --|7    8|-- !(G&H)
///          --------
/// ```
#[derive(Debug)]
pub struct GateNand {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for GateNand {
    fn default() -> Self {
        Self::new()
    }
}

impl GateNand {
    pub const TYPE: &'static str = "virt_ic::GateNand";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const NOT_A_AND_B: u8 = 3;
    pub const C: u8 = 4;
    pub const D: u8 = 5;
    pub const NOT_C_AND_D: u8 = 6;
    pub const E: u8 = 13;
    pub const F: u8 = 12;
    pub const NOT_E_AND_F: u8 = 11;
    pub const G: u8 = 10;
    pub const H: u8 = 9;
    pub const NOT_G_AND_H: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for GateNand {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate NAND",
            description: "A 4-in-one NAND gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B
            self.pin[2].borrow_mut().state = if self.pin[0].borrow().state == State::High
                && self.pin[1].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // C && D
            self.pin[5].borrow_mut().state = if self.pin[3].borrow().state == State::High
                && self.pin[4].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // E && F
            self.pin[10].borrow_mut().state = if self.pin[11].borrow().state == State::High
                && self.pin[12].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // G && H
            self.pin[7].borrow_mut().state = if self.pin[8].borrow().state == State::High
                && self.pin[9].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }
}

/// # A chip with 3 bundled "3-input NAND" gates
///
/// # Diagram
/// ```
///            ---__---
///        A --|1   14|-- VCC
///        B --|2   13|-- C
///        D --|3   12|-- !(A&B&C)
///        E --|4   11|-- G
///        F --|5   10|-- H
/// !(D&E&F) --|6    9|-- I
///      GND --|7    8|-- !(G&H&I)
///            --------
/// ```
#[derive(Debug)]
pub struct Gate3InputNand {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 14],
}
impl Default for Gate3InputNand {
    fn default() -> Self {
        Self::new()
    }
}

impl Gate3InputNand {
    pub const TYPE: &'static str = "virt_ic::Gate3InputNand";

    pub const A: u8 = 1;
    pub const B: u8 = 2;
    pub const C: u8 = 13;
    pub const NOT_A_AND_B_AND_C: u8 = 12;
    pub const D: u8 = 3;
    pub const E: u8 = 4;
    pub const F: u8 = 5;
    pub const NOT_D_AND_E_AND_F: u8 = 6;
    pub const G: u8 = 11;
    pub const H: u8 = 10;
    pub const I: u8 = 9;
    pub const NOT_G_AND_H_AND_I: u8 = 8;
    pub const VCC: u8 = 14;
    pub const GND: u8 = 7;

    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        Self {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 3, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 4, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 5, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Input))),
            ],
        }
    }
}
impl Chip for Gate3InputNand {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        14
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Gate 3-Input NAND",
            description: "A 3-in-one 3-Input NAND gate chip",
            data: String::new(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[6].borrow().state == State::Low && self.pin[13].borrow().state == State::High {
            // A && B && C
            self.pin[11].borrow_mut().state = if self.pin[0].borrow().state == State::High
                && self.pin[1].borrow().state == State::High
                && self.pin[12].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // D && E && F
            self.pin[5].borrow_mut().state = if self.pin[2].borrow().state == State::High
                && self.pin[3].borrow().state == State::High
                && self.pin[4].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
            // G && H && I
            self.pin[7].borrow_mut().state = if self.pin[10].borrow().state == State::High
                && self.pin[9].borrow().state == State::High
                && self.pin[8].borrow().state == State::High
            {
                State::Low
            } else {
                State::High
            };
        } else {
            // turn off every pin
            for i in 0..14 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }
}
