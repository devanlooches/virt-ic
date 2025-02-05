//! Readable and/or Writable Memory Chips
use super::{Chip, ChipInfo, Pin, PinType};
use crate::State;
use rand::random;
use std::cell::RefCell;
use std::rc::Rc;

/// # A 256-bytes RAM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// WE: Write Enable (active low)
/// OE: Output Enable (active low)
/// A0-7: Addresses
/// IO0-7: Input/Output
/// ```
///        ---__---
///  !CS --|1   22|-- VCC
///  !WE --|2   21|-- UNUSED
///  !OE --|3   20|-- IO7
///   A0 --|4   19|-- IO6
///   A1 --|5   18|-- IO5
///   A2 --|6   17|-- IO4
///   A3 --|7   16|-- IO3
///   A4 --|8   15|-- IO2
///   A5 --|9   14|-- IO1
///   A6 --|10  13|-- IO0
///  GND --|11  12|-- A7
///        --------
/// ```
pub struct Ram256B {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 22],
    ram: [u8; 256],
    powered: bool,
}
impl Default for Ram256B {
    fn default() -> Self {
        Self::new()
    }
}
impl ToString for Ram256B {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
---+------------------------------------------------",
        );
        for (addr, byte) in self.ram.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(" {byte:02X}"));
        }
        string
    }
}

impl Ram256B {
    pub const TYPE: &'static str = "virt_ic::Ram256B";

    pub const CS: u8 = 1;
    pub const WE: u8 = 2;
    pub const OE: u8 = 3;
    pub const A0: u8 = 4;
    pub const A1: u8 = 5;
    pub const A2: u8 = 6;
    pub const A3: u8 = 7;
    pub const A4: u8 = 8;
    pub const A5: u8 = 9;
    pub const A6: u8 = 10;
    pub const A7: u8 = 12;
    pub const IO0: u8 = 13;
    pub const IO1: u8 = 14;
    pub const IO2: u8 = 15;
    pub const IO3: u8 = 16;
    pub const IO4: u8 = 17;
    pub const IO5: u8 = 18;
    pub const IO6: u8 = 19;
    pub const IO7: u8 = 20;
    pub const VCC: u8 = 22;
    pub const GND: u8 = 11;

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
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 15, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 16, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 17, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 18, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 19, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 20, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 21, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 22, PinType::Input))),
            ],
            ram: [0; 256],
            powered: false,
        }
    }

    fn get_address(&self) -> u8 {
        let mut addr: u8 = 0;
        for i in 3..10 {
            let bit = u8::from(self.pin[i].borrow().state == State::High);
            addr += bit << (i - 3);
        }
        let bit = u8::from(self.pin[11].borrow().state == State::High);
        addr += bit << 7;
        addr
    }

    fn get_data(&self) -> u8 {
        let mut addr: u8 = 0;
        for i in 12..20 {
            let bit = u8::from(self.pin[i].borrow().state == State::High);
            addr += bit << (i - 12);
        }
        addr
    }
}
impl Chip for Ram256B {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }
    fn get_pin_qty(&self) -> u8 {
        22
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Ram 256 Bytes",
            description: "A Random Access Memory Chip that can contains 256 Bytes of data.
The data is not kept if the chip is no longer powered.",
            data: self.to_string(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[10].borrow().state == State::Low && self.pin[21].borrow().state == State::High {
            if !self.powered {
                for i in 0..256 {
                    self.ram[i] = random::<u8>();
                }
                self.powered = true;
            }
            // check Chip Select (active low)
            if self.pin[0].borrow().state == State::Low {
                //print!("RAM: selected\t");
                // check Write Enable (active low)
                if self.pin[1].borrow().state == State::Low {
                    // IO = Input
                    for i in 12..20 {
                        self.pin[i].borrow_mut().pin_type = PinType::Input;
                    }
                    // read data on IO pins
                    let addr = self.get_address() as usize;
                    //print!("RAM: write [{:02X}]: {:02X} \t", addr, self.get_data());
                    self.ram[addr] = self.get_data();
                }

                // check Output Enable (active low)
                if self.pin[2].borrow().state == State::Low {
                    // IO = Output
                    for i in 12..21 {
                        self.pin[i].borrow_mut().pin_type = PinType::Output;
                    }
                    // display data on IO pins
                    let addr = self.get_address() as usize;
                    //print!("RAM: read [{:02X}]: {:02X} \t", addr, self.ram[addr]);
                    self.pin[12].borrow_mut().state = State::from_u8(self.ram[addr], 0);
                    self.pin[13].borrow_mut().state = State::from_u8(self.ram[addr], 1);
                    self.pin[14].borrow_mut().state = State::from_u8(self.ram[addr], 2);
                    self.pin[15].borrow_mut().state = State::from_u8(self.ram[addr], 3);
                    self.pin[16].borrow_mut().state = State::from_u8(self.ram[addr], 4);
                    self.pin[17].borrow_mut().state = State::from_u8(self.ram[addr], 5);
                    self.pin[18].borrow_mut().state = State::from_u8(self.ram[addr], 6);
                    self.pin[19].borrow_mut().state = State::from_u8(self.ram[addr], 7);
                }
            //println!();
            } else {
                // IO : undefined
                for i in 12..20 {
                    self.pin[i].borrow_mut().pin_type = PinType::Undefined;
                }
            }
        } else if self.powered {
            // turn off every pin
            for i in 0..22 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
            self.powered = false;
        }
    }

    fn save_data(&self) -> Vec<String> {
        vec![
            ron::to_string(&self.ram.to_vec()).unwrap(),
            String::from(if self.powered { "ON" } else { "OFF" }),
        ]
    }
    fn load_data(&mut self, chip_data: &[String]) {
        let data: Vec<u8> = ron::from_str(&chip_data[0]).unwrap();
        self.ram.copy_from_slice(&data[..data.len()]);
        self.powered = chip_data[1] == "ON";
    }
}

/// # A 256-bytes ROM chip
///
/// # Diagram
/// CS: Chip Select (active low)
/// OE: Output Enable (active low)
/// A0-7: Addresses
/// IO0-7: Input/Output
/// ```
///         ---__---
///   !CS --|1   22|-- VCC
/// UNUSED--|2   21|-- UNUSED
///   !OE --|3   20|-- IO7
///    A0 --|4   19|-- IO6
///    A1 --|5   18|-- IO5
///    A2 --|6   17|-- IO4
///    A3 --|7   16|-- IO3
///    A4 --|8   15|-- IO2
///    A5 --|9   14|-- IO1
///    A6 --|10  13|-- IO0
///   GND --|11  12|-- A7
///         --------
/// ```
pub struct Rom256B {
    uuid: u128,
    pin: [Rc<RefCell<Pin>>; 22],
    rom: [u8; 256],
}
impl Default for Rom256B {
    fn default() -> Self {
        Self::new()
    }
}
impl ToString for Rom256B {
    fn to_string(&self) -> std::string::String {
        let mut string = String::from(
            "ADR| 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
---+------------------------------------------------",
        );
        for (addr, byte) in self.rom.iter().enumerate() {
            if addr % 16 == 0 {
                string.push_str(&format!("\n {addr:02X}|"));
            }
            string.push_str(&format!(" {byte:02X}"));
        }
        string
    }
}

impl Rom256B {
    pub const TYPE: &'static str = "virt_ic::Rom256B";

    pub const CS: u8 = 1;
    pub const OE: u8 = 3;
    pub const A0: u8 = 4;
    pub const A1: u8 = 5;
    pub const A2: u8 = 6;
    pub const A3: u8 = 7;
    pub const A4: u8 = 8;
    pub const A5: u8 = 9;
    pub const A6: u8 = 10;
    pub const A7: u8 = 12;
    pub const IO0: u8 = 13;
    pub const IO1: u8 = 14;
    pub const IO2: u8 = 15;
    pub const IO3: u8 = 16;
    pub const IO4: u8 = 17;
    pub const IO5: u8 = 18;
    pub const IO6: u8 = 19;
    pub const IO7: u8 = 20;
    pub const VCC: u8 = 22;
    pub const GND: u8 = 11;

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
                Rc::new(RefCell::new(Pin::new(uuid, 6, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 7, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 8, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 9, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 10, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 11, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 12, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 13, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 14, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 15, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 16, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 17, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 18, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 19, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 20, PinType::Output))),
                Rc::new(RefCell::new(Pin::new(uuid, 21, PinType::Input))),
                Rc::new(RefCell::new(Pin::new(uuid, 22, PinType::Input))),
            ],
            rom: [0; 256],
        }
    }

    pub fn from_data(data: [u8; 256]) -> Self {
        let mut rom = Self::new();
        rom.load_data(data);
        rom
    }

    pub fn load_data(&mut self, data: [u8; 256]) {
        self.rom.clone_from_slice(&data);
    }

    fn get_address(&self) -> u8 {
        let mut addr: u8 = 0;
        for i in 3..10 {
            let bit = u8::from(self.pin[i].borrow().state == State::High);
            addr += bit << (i - 3);
        }
        let bit = u8::from(self.pin[11].borrow().state == State::High);
        addr += bit << 7;
        addr
    }
}
impl Chip for Rom256B {
    fn get_uuid(&self) -> u128 {
        self.uuid
    }
    fn get_type(&self) -> &str {
        Self::TYPE
    }

    fn get_pin_qty(&self) -> u8 {
        22
    }

    fn _get_pin(&mut self, pin: u8) -> Rc<RefCell<Pin>> {
        self.pin[pin as usize - 1].clone()
    }

    fn get_info(&self) -> ChipInfo {
        ChipInfo {
            name: "Rom 256 Bytes",
            description: "A Real Only Memory Chip that can contains 256 Bytes of data.
The data is kept if the chip is no longer powered.",
            data: self.to_string(),
        }
    }

    fn run(&mut self, _: std::time::Duration) {
        // check alimented
        if self.pin[10].borrow().state == State::Low && self.pin[21].borrow().state == State::High {
            // check Chip Select (active low)
            if self.pin[0].borrow().state == State::Low {
                //print!("ROM: selected\t");
                // check Output Enable (active low)
                if self.pin[2].borrow().state == State::Low {
                    // IO = Output
                    for i in 12..21 {
                        self.pin[i].borrow_mut().pin_type = PinType::Output;
                    }
                    // display data on IO pins
                    let addr = self.get_address() as usize;
                    //print!("ROM: read [{:02X}]: {:02X} \t", addr, self.rom[addr]);
                    self.pin[12].borrow_mut().state = State::from_u8(self.rom[addr], 0);
                    self.pin[13].borrow_mut().state = State::from_u8(self.rom[addr], 1);
                    self.pin[14].borrow_mut().state = State::from_u8(self.rom[addr], 2);
                    self.pin[15].borrow_mut().state = State::from_u8(self.rom[addr], 3);
                    self.pin[16].borrow_mut().state = State::from_u8(self.rom[addr], 4);
                    self.pin[17].borrow_mut().state = State::from_u8(self.rom[addr], 5);
                    self.pin[18].borrow_mut().state = State::from_u8(self.rom[addr], 6);
                    self.pin[19].borrow_mut().state = State::from_u8(self.rom[addr], 7);
                }
            //println!();
            } else {
                // IO : undefined
                for i in 12..20 {
                    self.pin[i].borrow_mut().pin_type = PinType::Undefined;
                }
            }
        } else {
            // turn off every pin
            for i in 0..22 {
                self.pin[i].borrow_mut().state = State::Undefined;
            }
        }
    }

    fn save_data(&self) -> Vec<String> {
        vec![ron::to_string(&self.rom.to_vec()).unwrap()]
    }
    fn load_data(&mut self, chip_data: &[String]) {
        let data: Vec<u8> = ron::from_str(&chip_data[0]).unwrap();
        self.rom.copy_from_slice(&data[..data.len()]);
    }
}
