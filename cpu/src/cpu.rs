use std::{collections::HashMap, panic::Location, result};

use bitflags::Flags;

use crate::{
    bus::{Bus, Memory},
    opcodes::{OpCode, CB_PREFIXED_OPCODES_MAP, UNPREFIXED_OPCODES_MAP},
    registers::{CpuFlag, Registers},
};

pub struct Cpu {
    registers: Registers,
    bus: Bus,
}

impl Memory for Cpu {
    fn mem_read(&self, address: u16) -> u8 {
        self.bus.mem_read(address)
    }

    fn mem_read_16(&self, address: u16) -> u16 {
        self.bus.mem_read_16(address)
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.bus.mem_write(address, data)
    }

    fn mem_write_16(&mut self, address: u16, data: u16) {
        self.bus.mem_write_16(address, data);
    }
}

impl Cpu {
    pub fn new(registers: Registers, bus: Bus) -> Self {
        Cpu { registers, bus }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn handle_interrupt(&mut self) {
        todo!()
    }

    fn execute(&mut self, opcode: OpCode) -> u8 {
        match opcode.value {
            0x00 => opcode.tcycles.0,
            0x01 => self.ld_16(opcode),
            0x02 => self.ld_8(opcode),
            0x03 => self.inc_16(opcode),
            0x04 => self.inc_8(opcode),
            0x05 => self.dec_8(opcode),
            0x06 => self.ld_8(opcode),
            0x07 => self.rlca(opcode),
            0x08 => self.ld_16(opcode),
            0x09 => self.add_16(opcode),
            0x0A => self.ld_8(opcode),
            0x0B => self.dec_16(opcode),
            0x0C => self.inc_8(opcode),
            0x0D => self.dec_8(opcode),
            0x0E => self.ld_8(opcode),
            0x0F => self.rrca(opcode),

            0x11 => self.ld_16(opcode),
            0x12 => self.ld_8(opcode),
            0x16 => self.ld_8(opcode),
            0x1A => self.ld_8(opcode),
            0x1E => self.ld_8(opcode),
            0x21 => self.ld_16(opcode),
            0x22 => self.ld_8(opcode),
            0x26 => self.ld_8(opcode),
            0x2A => self.ld_8(opcode),
            0x2E => self.ld_8(opcode),
            0x31 => self.ld_16(opcode),
            0x32 => self.ld_8(opcode),
            0x36 => self.ld_8(opcode),
            0x3A => self.ld_8(opcode),
            0x3E => self.ld_8(opcode),

            0xF9 => self.ld_16(opcode),
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
    }

    pub fn run(&self) {
        todo!()
    }

    fn get_operands<'a>(&self, mnemonic: &'a str) -> &'a str {
        let operand: &str = mnemonic.split_whitespace().nth(1).unwrap_or_default();
        operand
    }

    fn ld_16(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "BC,u16" => {
                let value = self.fetch_word();
                self.registers.set_bc(value)
            }
            "(u16),SP" => {
                let address = self.fetch_word();
                self.mem_write_16(address, self.registers.sp);
            }
            "DE,u16" => {
                let value = self.fetch_word();
                self.registers.set_de(value)
            }
            "HL,u16" => {
                let value = self.fetch_word();
                self.registers.set_hl(value)
            }
            "SP,u16" => self.registers.sp = self.fetch_word(),
            "SP,HL" => self.registers.sp = self.registers.hl(),
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn ld_8(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "(BC),A" => self.mem_write(self.registers.bc(), self.registers.a),
            "B,u8" => self.registers.b = self.fetch_byte(),
            "A,(BC)" => self.registers.a = self.mem_read(self.registers.bc()),
            "C,u8" => self.registers.c = self.fetch_byte(),
            "(DE),A" => self.mem_write(self.registers.de(), self.registers.a),
            "D,u8" => self.registers.d = self.fetch_byte(),
            "A,(DE)" => self.registers.a = self.mem_read(self.registers.de()),
            "E,u8" => self.registers.e = self.fetch_byte(),
            "(HL+),A" => {
                let address = self.registers.increment_hl();
                self.mem_write(address, self.registers.a);
            }
            "H,u8" => self.registers.h = self.fetch_byte(),
            "A,(HL+)" => {
                let address = self.registers.increment_hl();
                self.registers.a = self.mem_read(address);
            }
            "L,u8" => self.registers.l = self.fetch_byte(),
            "(HL-),A" => {
                let address = self.registers.decrement_hl();
                self.mem_write(address, self.registers.a);
            }
            "(HL),u8" => {
                let value = self.fetch_byte();
                self.mem_write(self.registers.hl(), value);
            }
            "A,(HL-)" => {
                let address = self.registers.decrement_hl();
                self.registers.a = self.mem_read(address);
            }
            "A,u8" => self.registers.a = self.fetch_byte(),
            "B,B" => {}
            "B,C" => self.registers.b = self.registers.c,
            "B,D" => self.registers.b = self.registers.d,
            "B,E" => self.registers.b = self.registers.e,
            "B,H" => self.registers.b = self.registers.h,
            "B,L" => self.registers.b = self.registers.l,
            "B,(HL)" => self.registers.b = self.mem_read(self.registers.hl()),
            "B,A" => self.registers.b = self.registers.a,
            "C,B" => self.registers.c = self.registers.b,
            "C,C" => {}
            "C,D" => self.registers.c = self.registers.d,
            "C,E" => self.registers.c = self.registers.e,
            "C,H" => self.registers.c = self.registers.h,
            "C,L" => self.registers.c = self.registers.l,
            "C,(HL)" => self.registers.c = self.mem_read(self.registers.hl()),
            "C,A" => self.registers.c = self.registers.a,
            "D,B" => self.registers.d = self.registers.b,
            "D,C" => self.registers.d = self.registers.c,
            "D,D" => {}
            "D,E" => self.registers.d = self.registers.e,
            "D,H" => self.registers.d = self.registers.h,
            "D,L" => self.registers.d = self.registers.l,
            "D,(HL)" => self.registers.d = self.mem_read(self.registers.hl()),
            "D,A" => self.registers.d = self.registers.a,
            "E,B" => self.registers.e = self.registers.b,
            "E,C" => self.registers.e = self.registers.c,
            "E,D" => self.registers.e = self.registers.d,
            "E,E" => {}
            "E,H" => self.registers.e = self.registers.h,
            "E,L" => self.registers.e = self.registers.l,
            "E,(HL)" => self.registers.e = self.mem_read(self.registers.hl()),
            "E,A" => self.registers.e = self.registers.a,
            "H,B" => self.registers.h = self.registers.b,
            "H,C" => self.registers.h = self.registers.c,
            "H,D" => self.registers.h = self.registers.d,
            "H,E" => self.registers.h = self.registers.e,
            "H,H" => {}
            "H,L" => self.registers.h = self.registers.l,
            "H,(HL)" => self.registers.h = self.mem_read(self.registers.hl()),
            "H,A" => self.registers.h = self.registers.a,
            "L,B" => self.registers.l = self.registers.b,
            "L,C" => self.registers.l = self.registers.c,
            "L,D" => self.registers.l = self.registers.d,
            "L,E" => self.registers.l = self.registers.e,
            "L,H" => self.registers.l = self.registers.h,
            "L,L" => {}
            "L,(HL)" => self.registers.l = self.mem_read(self.registers.hl()),
            "L,A" => self.registers.l = self.registers.a,
            "(HL),B" => self.mem_write(self.registers.hl(), self.registers.b),
            "(HL),C" => self.mem_write(self.registers.hl(), self.registers.c),
            "(HL),D" => self.mem_write(self.registers.hl(), self.registers.d),
            "(HL),E" => self.mem_write(self.registers.hl(), self.registers.e),
            "(HL),H" => self.mem_write(self.registers.hl(), self.registers.h),
            "(HL),L" => self.mem_write(self.registers.hl(), self.registers.l),
            "(HL),A" => self.mem_write(self.registers.hl(), self.registers.a),
            "A,B" => self.registers.a = self.registers.b,
            "A,C" => self.registers.a = self.registers.c,
            "A,D" => self.registers.a = self.registers.d,
            "A,E" => self.registers.a = self.registers.e,
            "A,H" => self.registers.a = self.registers.h,
            "A,L" => self.registers.a = self.registers.l,
            "A,(HL)" => self.registers.a = self.mem_read(self.registers.hl()),
            "A,A" => {}
            "(FF00 + u8),A" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.mem_write(address, self.registers.a);
            }
            "(FF00 + C),A" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.mem_write(address, self.registers.a);
            }
            "(u16),A" => {
                let address = self.fetch_word();
                self.mem_write(address, self.registers.a)
            }
            "A,(FF00 + u8)" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.registers.a = self.mem_read(address)
            }
            "A,(FF00 + C)" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.registers.a = self.mem_read(address)
            }
            "A,(u16)" => {
                let address = self.fetch_word();
                self.registers.a = self.mem_read(address)
            }
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn inc_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_add(1)),
            "DE" => self.registers.set_de(self.registers.de().wrapping_add(1)),
            "HL" => self.registers.set_hl(self.registers.hl().wrapping_add(1)),
            "SP" => self.registers.sp = self.registers.sp.wrapping_add(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn inc_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = self.registers.b.wrapping_add(1);
                result = self.registers.b;
            }
            "C" => {
                data = self.registers.c;
                self.registers.c = self.registers.c.wrapping_add(1);
                result = self.registers.c;
            }
            "D" => {
                data = self.registers.d;
                self.registers.d = self.registers.d.wrapping_add(1);
                result = self.registers.d;
            }
            "E" => {
                data = self.registers.e;
                self.registers.e = self.registers.e.wrapping_add(1);
                result = self.registers.e;
            }
            "H" => {
                data = self.registers.h;
                self.registers.h = self.registers.h.wrapping_add(1);
                result = self.registers.h;
            }
            "L" => {
                data = self.registers.l;
                self.registers.l = self.registers.l.wrapping_add(1);
                result = self.registers.l;
            }
            "(HL)" => {
                data = self.mem_read(self.registers.hl());
                self.mem_write(
                    self.registers.hl(),
                    self.mem_read(self.registers.hl()).wrapping_add(1),
                );
                result = self.mem_read(self.registers.hl());
            }
            "A" => {
                data = self.registers.a;
                self.registers.a = self.registers.a.wrapping_add(1);
                result = self.registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) + 1 > 0x0F);

        opcode.tcycles.0
    }

    fn dec_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_sub(1)),
            "DE" => self.registers.set_de(self.registers.de().wrapping_sub(1)),
            "HL" => self.registers.set_hl(self.registers.hl().wrapping_sub(1)),
            "SP" => self.registers.sp = self.registers.sp.wrapping_sub(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn dec_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = self.registers.b.wrapping_sub(1);
                result = self.registers.b;
            }
            "C" => {
                data = self.registers.c;
                self.registers.c = self.registers.c.wrapping_sub(1);
                result = self.registers.c;
            }
            "D" => {
                data = self.registers.d;
                self.registers.d = self.registers.d.wrapping_sub(1);
                result = self.registers.d;
            }
            "E" => {
                data = self.registers.e;
                self.registers.e = self.registers.e.wrapping_sub(1);
                result = self.registers.e;
            }
            "H" => {
                data = self.registers.h;
                self.registers.h = self.registers.h.wrapping_sub(1);
                result = self.registers.h;
            }
            "L" => {
                data = self.registers.l;
                self.registers.l = self.registers.l.wrapping_sub(1);
                result = self.registers.l;
            }
            "(HL)" => {
                data = self.mem_read(self.registers.hl());
                self.mem_write(
                    self.registers.hl(),
                    self.mem_read(self.registers.hl()).wrapping_sub(1),
                );
                result = self.mem_read(self.registers.hl());
            }
            "A" => {
                data = self.registers.a;
                self.registers.a = self.registers.a.wrapping_sub(1);
                result = self.registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, true);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) == 0);

        opcode.tcycles.0
    }

    fn add_16(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "HL,BC" => {
                (data1, data2) = (self.registers.hl(), self.registers.bc());
                let result = self.registers.hl().wrapping_add(self.registers.bc());
                self.registers.set_hl(result);
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(
            CpuFlag::HALF_CARRY,
            (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF,
        );
        self.registers
            .set_flag(CpuFlag::CARRY, data1 > 0xFFFF - data2);
        opcode.tcycles.0
    }

    fn rlca(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x01
        } else {
            0
        };

        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    fn rrca(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x01 == 0x01);

        let first_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x80
        } else {
            0
        };

        self.registers.a = first_bit | self.registers.a >> 1;

        opcode.tcycles.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::opcodes;
    use utils::Mode;

    fn get_cpu() -> Cpu {
        let registers = Registers::new(Mode::Monochrome);
        let bus = Bus::new();
        let cpu = Cpu::new(registers, bus);
        cpu
    }

    #[test]
    fn execute_nop() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x00];
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4)
    }

    #[test]
    fn execute_ld_bc_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x01];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.bc(), 0x4423);
    }

    #[test]
    fn execute_ld_value_at_bc_with_a() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x02];
        cpu.registers.a = 0x44;

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.mem_read(cpu.registers.bc()), 0x44);
    }

    #[test]
    fn execute_inc_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x03];
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4545);
    }

    #[test]
    fn execute_inc_b() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x04];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x45;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x46);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0b0001_1111;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x20);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0xFF;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1010_0000);
    }

    #[test]
    fn execute_dec_b() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x05];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x31;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x30);
        assert_eq!(cpu.registers.f.bits(), 0b0100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x01;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.f.bits(), 0b0110_0000);
    }

    #[test]
    fn execute_ld_b_with_u8() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x06];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.b, 0x23);
    }

    #[test]
    fn execute_rlca() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x07];

        cpu.registers.a = 0x44;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0x88);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);
        assert_eq!(tcylcles, 4);

        cpu.registers.a = 0x88;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.f.bits(), 0b0001_0000);
        assert_eq!(tcylcles, 4);
    }

    #[test]
    fn execute_ld_u16_with_sp() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x08];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);
        cpu.registers.sp = 0x5555;

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 20);
        assert_eq!(cpu.mem_read_16(0x4423), 0x5555);
    }

    #[test]
    fn execute_add_hl_with_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x09];

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x7CFF);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.set_hl(0x07FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x83FF);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0xFF01);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.bits(), 0b0011_0000);
    }

    #[test]
    fn execute_ld_a_with_value_at_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0A];
        cpu.mem_write(cpu.registers.bc(), 0x44);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.a, 0x44);
    }

    #[test]
    fn execute_dec_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0B];
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4543);
    }

    #[test]
    fn execute_inc_c() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0C];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x45;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x46);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0b0001_1111;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x20);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0xFF;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1010_0000);
    }

    #[test]
    fn execute_dec_c() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0D];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x31;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x30);
        assert_eq!(cpu.registers.f.bits(), 0b0100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x01;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0xFF);
        assert_eq!(cpu.registers.f.bits(), 0b0110_0000);
    }

    #[test]
    fn execute_ld_c_with_u8() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0E];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.c, 0x23);
    }

    #[test]
    fn execute_rrca() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0F];

        cpu.registers.a = 0x44;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0x22);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);
        assert_eq!(tcylcles, 4);

        cpu.registers.a = 0x89;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0xC4);
        assert_eq!(cpu.registers.f.bits(), 0b0001_0000);
        assert_eq!(tcylcles, 4);
    }

    #[test]
    fn execute_ld_de_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x11];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.de(), 0x4423);
    }

    #[test]
    fn execute_ld_hl_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x21];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.hl(), 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x31];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.sp, 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_hl() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0xF9];
        cpu.registers.set_hl(0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.sp, 0x4423);
    }
}
