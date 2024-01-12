use crate::cartridge::Mirroring;
use registers::addr::AddrRegister;
use registers::control::ControlRegister;
use registers::status::StatusRegister;

pub mod registers;

pub struct NesPPU {
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    pub status: StatusRegister,
    pub addr: AddrRegister,
    pub vram: [u8; 2048],

    pub oam_addr: u8,
    pub palette_table: [u8; 32],

    internal_data_buf: u8,

    pub scanline: u16,
    cycles: usize,
    pub nmi_interrupt: Option<u8>,
}

pub trait PPU
 {
    fn write_to_ctrl(&mut self, value: u8);
    fn write_to_mask(&mut self, value: u8);
    fn read_status(&mut self) -> u8;
    fn write_to_oam_addr(&mut self, value: u8);
    fn write_to_oam_data(&mut self, value: u8);
    fn read_oam_data(&self) -> u8;
    fn write_to_scroll(&mut self, value: u8);
    fn write_to_ppu_addr(&mut self, value: u8);
    fn write_to_data(&mut self, value: u8);
    fn read_data(&mut self) -> u8;
    fn write_oam_dma(&mut self, value: &[u8; 256]);
}

impl NesPPU {
    pub fn new_empty_rom() -> Self 
    {
        NesPPU::new(vec![0; 2048], Mirroring::HORIZONTAL)
    }

    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self
     {
        NesPPU 
        {
            chr_rom: chr_rom,
            mirroring: mirroring,
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            oam_addr: 0,
            scroll: ScrollRegister::new(),
            addr: AddrRegister::new(),
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            palette_table: [0; 32],
            internal_data_buf: 0,
        }
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; 
        let vram_index = mirrored_vram - 0x2000; 
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table) {
            (Mirroring::VERTICAL, 2) | (Mirroring::VERTICAL, 3) => vram_index - 0x800,
            (Mirroring::HORIZONTAL, 2) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 1) => vram_index - 0x400,
            (Mirroring::HORIZONTAL, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            if self.is_sprite_0_hit(self.cycles) {
                self.status.set_sprite_zero_hit(true);
            }

            self.cycles = self.cycles - 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.status.set_vblank_status(true);
                self.status.set_sprite_zero_hit(false);
                if self.ctrl.generate_vblank_nmi() {
                    self.nmi_interrupt = Some(1);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.nmi_interrupt = None;
                self.status.set_sprite_zero_hit(false);
                self.status.reset_vblank_status();
                return true;
            }
        }
        return false;
    }

    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;
        (y == self.scanline as usize) && x <= cycle && self.mask.show_sprites()
    }

}

impl PPU for NesPPU {
    fn write_to_ctrl(&mut self, value: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(value);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    fn write_to_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    fn write_to_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    fn write_to_data(&mut self, value: u8) {
        let addr = self.addr.get();
        match addr {
            0..=0x1fff => println!("attempt to write to chr rom space {}", addr),
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = value;
            }
            0x3000..=0x3eff => unimplemented!("addr {} shouldn't be used in reallity", addr),
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = value;
            }
        }
        self.increment_vram_addr();
    }

    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();

        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }

            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.oam_data[self.oam_addr as usize] = *x;
            self.oam_addr = self.oam_addr.wrapping_add(1);
        }
    }
}

