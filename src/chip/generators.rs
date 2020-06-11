//! Generators that provide fixed currents
use crate::save::SavedChip;
use crate::State;
use super::{Pin, PinType, Chip};
use std::cell::RefCell;
use std::rc::Rc;

/// # A simple generator providing VCC and GND
/// 
/// # Diagram
/// ```
///        --------
///  VCC --|1    2|-- GND
///        --------
/// ```
#[derive(Debug)]
pub struct Generator {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 2],
}
impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    pub const VCC: u8 = 1;
    pub const GND: u8 = 2;
    
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4().as_u128();
        let gen = Generator {
            uuid,
            pin: [
                Rc::new(RefCell::new(Pin::new(uuid, 1, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 2, PinType::Output))),
            ]
        };
        gen.pin[0].borrow_mut().state = State::High;
        gen.pin[1].borrow_mut().state = State::Low;
        gen
    }
}
impl Chip for Generator {
    fn get_uuid(&self) -> u128 {
        self.uuid
    } 
    fn get_type(&self) -> &str {
        "virt_ic::Generator"
    }
    fn get_pin_qty(&self) -> u8 { 
        2
    }

    fn get_pin(&mut self, pin: u8) -> Result<Rc<RefCell<Pin>>, &str> { 
        if pin > 0 && pin <= 2 {
            Ok(self.pin[pin as usize-1].clone())
        } else {
            Err("Pin out of bounds")
        }
    }
    fn run(&mut self, _: std::time::Duration) {
        self.pin[0].borrow_mut().state = State::High;
        self.pin[1].borrow_mut().state = State::Low;
    }

    fn save(&self) -> SavedChip {
        SavedChip {
            uuid: self.uuid,
            chip_type: String::from(self.get_type()),
            chip_data: vec![]
        }
    }

    fn load(&mut self, _s_chip: &SavedChip) {}
}