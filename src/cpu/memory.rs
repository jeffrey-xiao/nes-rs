use cpu::Cpu;
use mapper::Mapper;
use ppu::Ppu;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Memory([u8; 0x800]);

impl Memory {
    pub fn new() -> Self {
        Memory([0; 0x800])
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        self.0[index]
    }

    pub fn write_byte(&mut self, index: usize, val: u8) {
        self.0[index] = val;
    }
}

pub struct MemoryMap {
    memory: Memory,
    cpu: Weak<RefCell<Cpu>>,
    ppu: Weak<RefCell<Ppu>>,
    mapper: Weak<RefCell<Box<Mapper>>>,
}

impl MemoryMap {
    pub fn new(
        cpu: &Rc<RefCell<Cpu>>,
        ppu: &Rc<RefCell<Ppu>>,
        mapper: &Rc<RefCell<Box<Mapper>>>,
    ) -> Self {
        MemoryMap {
            memory: Memory::new(),
            cpu: Rc::downgrade(cpu),
            ppu: Rc::downgrade(ppu),
            mapper: Rc::downgrade(mapper),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.memory.read_byte((addr % 0x0800) as usize),
            0x2000..=0x3FFF => {
                let ppu = self.ppu.upgrade().unwrap();
                let ret = ppu.borrow().read_register(((addr - 0x2000) % 8) as usize);
                drop(ppu);
                ret
            },
            0x4000..=0x4017 => panic!("APU and IO registers not implemented."),
            0x4018..=0x401F => panic!("CPU Test Mode not implemented."),
            0x4020..=0xFFFE => {
                let mapper = self.mapper.upgrade().unwrap();
                let ret = mapper.borrow().read_byte(addr);
                drop(mapper);
                ret
            },
            _ => panic!("Invalid memory address: {:#6x}.", addr),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        ((self.read_byte(addr + 1) as u16) << 8) | self.read_byte(addr) as u16
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.memory.write_byte((addr % 0x0800) as usize, val),
            0x2000..=0x3FFF => {
                let mut ppu = self.ppu.upgrade().unwrap();
                ppu.borrow_mut()
                    .write_register(((addr - 0x2000) % 8) as usize, val);
            },
            0x4000..=0x4017 => panic!("APU and IO registers not implemented."),
            0x4018..=0x401F => panic!("CPU Test Mode not implemented."),
            0x4020..=0xFFFE => {
                let mut mapper = self.mapper.upgrade().unwrap();
                mapper.borrow_mut().write_byte(addr, val);
            },
            _ => panic!("Invalid memory address: {:#6x}.", addr),
        }
    }
}
