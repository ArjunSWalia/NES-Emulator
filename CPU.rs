use crate::opcodes;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.mem_write(pos, (data & 0xff) as u8);
        self.mem_write(pos + 1, (data >> 8) as u8);
    }
}

impl Mem for CPU
{
    fn mem_read(&self, addr: u16) -> u8 { 
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) { 
        self.memory[addr as usize] = data;
    }
}


fn lda(&mut self, value: u8) {
    self.register_a = value;
    self.update_zero_and_negative_flags(self.register_a);
}

fn tax(&mut self) {
    self.register_x = self.register_a;
    self.update_zero_and_negative_flags(self.register_x);
}

fn update_zero_and_negative_flags(&mut self, result: u8) {
    if result == 0 {
        self.status = self.status | 0b0000_0010;
    } else {
        self.status = self.status & 0b1111_1101;
    }

    if result & 0b1000_0000 != 0 {
        self.status = self.status | 0b1000_0000;
    } else {
        self.status = self.status & 0b0111_1111;
    }
}


pub fn interpret(&mut self, program: Vec<u8>) {
    self.program_counter = 0;

    loop {
        let opscode = program[self.program_counter as usize];
        self.program_counter += 1;

        match opscode {
            _ => todo!()
        }
    }
}

pub fn run(&mut self) {
    let ref opcodes: 'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

    loop {
        let code = self.mem_read(self.program_counter);
        self.program_counter++;

        let opcode = opcodes.get(code).expect(&format!("ERROR", code));

        match code {
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                self.lda(&opcode.mode);
            }
            
            0xAA => self.tax(),
            0xe8 => self.inx(),
            0x00 => return,
            _ => todo!(),
        }


match opscode {
    0xA9 => {
        let param = program[self.program_counter as usize];
        self.program_counter +=1;
        self.register_a = param;

        if self.register_a == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if self.register_a & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }

    }
    _ => todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);

    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }


pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
 
        self.program_counter = self.mem_read_u16(0xFFFC);
    }


    impl CPU {
        // ...
        fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
     
            match mode {
                AddressingMode::Immediate => self.program_counter,
     
                AddressingMode::ZeroPage  => self.mem_read(self.program_counter) as u16,
               
                AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
             
                AddressingMode::ZeroPage_X => {
                    let pos = self.mem_read(self.program_counter);
                    let addr = pos.wrapping_add(self.register_x) as u16;
                    addr
                }
                AddressingMode::ZeroPage_Y => {
                    let pos = self.mem_read(self.program_counter);
                    let addr = pos.wrapping_add(self.register_y) as u16;
                    addr
                }
            }
        }
    }     
 