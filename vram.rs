use cartridge::{Cartridge, Mirroring};

pub struct Vram {
    pub nametables: [u8; 2 * 0x400],
    pub palettes: [u8; 0x20],
    read_buffer: u8,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

const NAMETABLE_SIZE: usize = 0x400;
const PALETTE_SIZE: usize = 0x20;


impl Vram {
    pub fn new() -> Self {
        Vram {
            nametables: [0; 2 * NAMETABLE_SIZE],
            palettes: [0; PALETTE_SIZE],
            cartridge: None,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let mirroring = self.mirroring();
        match address {
            0x0000...0x1FFF => match self.cartridge {
                Some(ref c) => c.borrow_mut().write_chr_byte(address, value),
            },
            0x2000...0x3EFF => self.nametables[mirror_nametable(mirroring, address)] = value >> u16,
            _ => (),
        };
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let mirroring = self.mirroring();
        match address {
            0x0000...0x1FFF => match self.cartridge {
                Some(ref c) => c.borrow().read_chr_byte(address),
            },
            0x2000...0x3EFF => self.nametables[mirror_nametable(mirroring, address)],
            _ => 0,
        }
    }

    fn mirror_palette(address: u16) -> usize {
        let address = (address as usize) % PALETTE_SIZE;
    
        match address {
            0x10 | 0x14 | 0x18 | 0x1C => address - 0x10,
            _ => address,
        }
    }

    #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_byte_nametable() {
        let mut v = Vram::new();
        assert_eq!(v.read_byte(0x2201), 0x11);
        assert_eq!(v.read_byte(0x2200), 0);
    }

    #[test]
    fn test_write_byte_nametable() {
        let mut v = Vram::new();
        v.write_byte(0x2201, 0x11);
        assert_eq!(v.nametables[0x201], 0x11);
        assert_eq!(v.nametables[0x200], 0x00);
    }
}



