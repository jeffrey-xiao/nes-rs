use std::cmp;

pub struct Controller {
    // A, B, Select, Start, Up, Down, Left, Right
    value: u8,
    index: u8,
    strobe: bool,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            value: 0,
            index: 0,
            strobe: false,
        }
    }

    pub fn write_strobe(&mut self, val: bool) {
        self.strobe = val;
        if self.strobe {
            self.index = 0;
        }
    }

    pub fn read_value(&mut self) -> u8 {
        let ret = (self.value.wrapping_shr(self.index as u32)) & 0x01;
        self.index = cmp::min(self.index + 1, 8);
        if self.strobe {
            self.index = 0;
        }
        println!("index is {}", self.index);
        ret
    }

    pub fn press_button(&mut self, index: u8) {
        self.value |= 1 << index;
    }

    pub fn release_button(&mut self, index: u8) {
        self.value &= !(1 << index);
    }
}

impl Default for Controller {
    fn default() -> Self {
        Controller::new()
    }
}
