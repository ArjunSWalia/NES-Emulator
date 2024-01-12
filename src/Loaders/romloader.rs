use std::path::Path;




trait Mapper {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

struct NromMapper {
    prg_rom: Vec<u8>,
    save_ram: Vec<u8>,
}

impl NromMapper {
    fn new(prg_rom: Vec<u8>, save_ram_size: usize) -> Self {
        NromMapper {
            prg_rom,
            save_ram: vec![0; save_ram_size], // zero RAM
        }
    }
}

impl Mapper for NromMapper {
    fn read(&self, address: u16) -> u8 {
        match address 
        {
            0x6000..=0x7FFF => 
            {
                let index = (address - 0x6000) as usize;
                self.save_ram[index]
            },
            0x8000..=0xFFFF => {
                let addr = if self.prg_rom.len() > 16 * 1024 
                {
                    (address - 0x8000) as usize
                } else 
                {
                    (address - 0x8000) as usize % 16 * 1024
                };
                self.prg_rom[addr]
            },
            _ => 0, //null addresses
        }
    }
    fn write(&mut self, address: u16, value: u8)
     {
        match address {

            0x6000..=0x7FFF => 
            {
                let index = (address - 0x6000) as usize;
                self.save_ram[index] = value;
            },
            _ => {},
        }
    }
}


class ROM
{
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: Box<dyn Mapper>
}

class INesHeader
{
    signature: [u8; 4],
    prg_rom_size: u8,
    chr_rom_size: u8,
    flags6: u8,
    flags7: u8,
    padding: [u8; 8],
}

impl Cartridge {
    fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> 
    {
        let mut file = File::open(path)?;

        let mut header_bytes = [0u8; 16];
        file.read_exact(&mut header_bytes)?;
        let header = unsafe { std::ptr::read(header_bytes.as_ptr() as *const _) };

        if header.signature != [0x4E, 0x45, 0x53, 0x1A] 
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid NES file"));
        }
        let mut prg_rom = vec![0u8; header.prg_rom_size as usize * 16384];
        file.read_exact(&mut prg_rom)?;
        let mut chr_rom = vec![0u8; header.chr_rom_size as usize * 8192]; 
        Ok(Cartridge { prg_rom, chr_rom })
    }
}





fn main()-> io::Result<()> {
    let cartridge = Cartridge::load("...nes")?;
    Ok(())
}
