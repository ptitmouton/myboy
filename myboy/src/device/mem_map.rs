// 0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
// 4000	7FFF	16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
// 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
// A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
// C000	CFFF	4 KiB Work RAM (WRAM)
// D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
// E000	FDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
// FE00	FE9F	Object attribute memory (OAM)
// FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited.
// FF00	FF7F	I/O Registers
// FF80	FFFE	High RAM (HRAM)
// FFFF	FFFF	Interrupt Enable register (IE)

use crate::{
    io::io_registers::IORegisters,
    memory::{generic_memory::GenericMemory as _, hram::HRAM, vram::VRAM, wram::WRAM},
    ppu::oam::OAM,
};
use mygbcartridge::{cartridge::Cartridge, enums::cartridge_type::CartridgeType};

pub struct MemMap {
    pub(crate) cartridge: Cartridge,
    pub working_ram: WRAM,
    pub video_ram: VRAM,
    pub io_registers: IORegisters,
    pub object_attribute_memory: OAM,
    pub hram: HRAM,
}

impl MemMap {
    pub fn new(cartridge: Cartridge) -> MemMap {
        let working_ram = WRAM::new();
        let video_ram = VRAM::new();
        let io_registers = IORegisters::new();
        let object_attribute_memory = OAM::new();
        let hram = HRAM::new();

        MemMap {
            cartridge,
            working_ram,
            video_ram,
            io_registers,
            object_attribute_memory,
            hram,
        }
    }

    pub fn io(&self) -> &IORegisters {
        &self.io_registers
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address),
            0x8000..=0x9FFF => self.video_ram.read_byte(address),
            0xA000..=0xBFFF => 0x00, // todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_byte(address),
            0xE000..=0xFDFF => self.working_ram.read_byte(address - 0x2000),
            0xFE00..=0xFE9F => self.object_attribute_memory.read_byte(address),
            0xFEA0..=0xFEFF => 0x00, // todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => self.io_registers.read_byte(address),
            0xFF80..=0xFFFE => self.hram.read_byte(address),
            0xFFFF => self.io_registers.read_byte(address),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_word(address),
            0x8000..=0x9FFF => self.video_ram.read_word(address),
            0xA000..=0xBFFF => 0x0000, // todo!("Read from External RAM"),
            0xC000..=0xDFFF => self.working_ram.read_word(address),
            0xE000..=0xFDFF => self.working_ram.read_word(address - 0x2000),
            0xFE00..=0xFE9F => self.object_attribute_memory.read_word(address),
            0xFEA0..=0xFEFF => 0x0000, // todo!("Read from Not Usable"),
            0xFF00..=0xFF7F => u16::from_le_bytes([
                self.io_registers.read_byte(address),
                self.io_registers.read_byte(address + 1),
            ]),
            0xFF80..=0xFFFE => self.hram.read_word(address),
            0xFFFF => panic!("Cannot read words from interrupts"),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => match self.cartridge.get_cartridge_type().unwrap() {
                CartridgeType::RomRam
                | CartridgeType::Mbc1Ram
                | CartridgeType::Mbc1RamBattery
                | CartridgeType::Mmm01Ram
                | CartridgeType::Mmm01RamBattery
                | CartridgeType::Mbc3Ram
                | CartridgeType::Mbc5Ram
                | CartridgeType::Mbc5RumbleRam
                | CartridgeType::Mbc5RumbleRamBattery
                | CartridgeType::Mbc7SensorRumbleRamBattery
                | CartridgeType::HuC1RamBattery => {
                    panic!("Cartridge ram handling not implemented");
                }
                _ => {}
            },
            0x8000..=0x9FFF => self.video_ram.write_byte(address, value),
            0xC000..=0xDFFF => self.working_ram.write_byte(address, value),
            0xFE00..=0xFE9F => self.object_attribute_memory.write_byte(address, value),
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.io_registers.write_byte(address, value),
            0xFF80..=0xFFFE => self.hram.write_byte(address, value),
            0xFFFF => {
                self.io_registers.ie_register.0 = value;
            }
            _ => todo!("Write to memory map 0x{:04x}", address),
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        match address {
            0x0000..=0x7FFF => match self.cartridge.get_cartridge_type().unwrap() {
                CartridgeType::RomRam
                | CartridgeType::Mbc1Ram
                | CartridgeType::Mbc1RamBattery
                | CartridgeType::Mmm01Ram
                | CartridgeType::Mmm01RamBattery
                | CartridgeType::Mbc3Ram
                | CartridgeType::Mbc5Ram
                | CartridgeType::Mbc5RumbleRam
                | CartridgeType::Mbc5RumbleRamBattery
                | CartridgeType::Mbc7SensorRumbleRamBattery
                | CartridgeType::HuC1RamBattery => {
                    panic!("Cartridge ram handling not implemented");
                }
                _ => {}
            },
            0x8000..=0x9FFF => self.video_ram.write_word(address, value),
            0xC000..=0xDFFF => self.working_ram.write_word(address, value),
            0xFE00..=0xFE9F => self.object_attribute_memory.write_word(address, value),
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => panic!("Cannot write word to I/O registers"),
            0xFF80..=0xFFEF => self.hram.write_word(address, value),
            0xFFFF => panic!("Cannot write word to interrupts"),
            _ => todo!("Write to memory map: 0x{:04x}", address),
        }
    }
}
