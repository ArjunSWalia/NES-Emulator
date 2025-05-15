use crate::cpu::AddressingMode;
use crate::cpu::Mem;
use crate::cpu::CPU;
use crate::opcodes;
use crate::Bus;


pub fn trace(cpu: CPU) -> String {

    let code = cpu.mem_read(cpu.program_counter);
    let ops = opscodes.get(&code).unwrap();

    let begin = cpu.program_counter;
    let mut hex_dump = vec![];

    let (mem_addr, stored_value) = match ops.mode {
        AddressingMode::Immediate | AddressingMode::NoneAddressing => (0, 0),
        _ => {
            let addr = cpu.get_absolute_address(&ops.mode,begin +1);
            (addr, cpu.mem_read(addr))
        }
    };

    let tmp = match ops.len {
        1 => match ops.code {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },


        2 => {
            let address: u8 = cpu.mem_read(begin + 1);
            // let value = cpu.mem_read(address));
            hex_dump.push(address);

            match ops.mode {
                AddressingMode::Immediate => format!("#${:02x}", address),
                AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                AddressingMode::ZeroPage_X => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
            }
        }

        3 => {
            let address_lo = cpu.mem_read(begin + 1);
            let address_hi = cpu.mem_read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.mem_read_u16(begin + 1);

            match ops.mode {
                AddressingMode::NoneAddressing => {
                    if ops.code == 0x6c {
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.mem_read(address);
                            let hi = cpu.mem_read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.mem_read_u16(address)
                        };
                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                }
                AddressingMode::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                AddressingMode::Absolute_X => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                AddressingMode::Absolute_Y => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    ops.mode, ops.code
                ),
            }
        }
        _ => String::from(""),
    };

    #[test]
    fn test_format_mem_access() {
        let mut bus = Bus::new(test_rom());


        bus.mem_write(100, 0x11);
        bus.mem_write(101, 0x33);
        bus.mem_write(0x400, 0xAA);

        let mut cpu = CPU::new(bus);
        cpu.program_counter = 0x64;
        cpu.register_y = 0;
        let mut result: Vec<String> = vec![];
        cpu.run_with_callback(|cpu| {
            result.push(trace(cpu));
        });
    }

}