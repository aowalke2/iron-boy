use crate::cpu::Cpu;

use super::R8;

pub fn bit_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let result = data & (1 << (bit_index)) == 0;
    cpu.registers.f.z = result;
    cpu.registers.f.n = false;
    cpu.registers.f.h = true;
    if register == R8::HLMem {
        12
    } else {
        8
    }
}

pub fn res_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, data & !(1 << bit_index));
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn set_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, data | (1 << bit_index));
    if register == R8::HLMem {
        16
    } else {
        8
    }
}
