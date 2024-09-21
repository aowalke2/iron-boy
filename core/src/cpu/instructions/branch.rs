use crate::cpu::{registers::CpuFlag, Cpu};

use super::Condition;

pub fn jr_imm8(cpu: &mut Cpu) -> u8 {
    let signed = cpu.fetch_byte() as i8;
    cpu.registers.pc = ((cpu.registers.pc as i16) + (signed as i16)) as u16;
    12
}

pub fn jr_cond_imm8(cpu: &mut Cpu) -> u8 {
    let z = cpu.registers.f.contains(CpuFlag::Z);
    let c = cpu.registers.f.contains(CpuFlag::C);

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if jump {
        let signed = cpu.fetch_byte() as i8;
        cpu.registers.pc = ((cpu.registers.pc as i16) + (signed as i16)) as u16;
        12
    } else {
        cpu.registers.pc += 1;
        8
    }
}

pub fn jp_cond_imm16(cpu: &mut Cpu) -> u8 {
    let z = cpu.registers.f.contains(CpuFlag::Z);
    let c = cpu.registers.f.contains(CpuFlag::C);

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if jump {
        cpu.registers.pc = cpu.fetch_word();
        16
    } else {
        cpu.registers.pc += 2;
        12
    }
}

pub fn jp_imm16(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.fetch_word();
    16
}

pub fn jp_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.registers.hl();
    4
}

pub fn ret_cond(cpu: &mut Cpu) -> u8 {
    let z = cpu.registers.f.contains(CpuFlag::Z);
    let c = cpu.registers.f.contains(CpuFlag::C);

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let ret = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if ret {
        cpu.registers.pc = cpu.pop_stack();
        20
    } else {
        8
    }
}

pub fn ret(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.pop_stack();
    16
}

pub fn reti(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.pop_stack();
    cpu.interrupts.set_ime(true);
    16
}

pub fn call_cond_imm16(cpu: &mut Cpu) -> u8 {
    let z = cpu.registers.f.contains(CpuFlag::Z);
    let c = cpu.registers.f.contains(CpuFlag::C);

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;

    let call = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if call {
        cpu.push_stack(cpu.registers.pc + 2);
        cpu.registers.pc = cpu.fetch_word();
        24
    } else {
        cpu.registers.pc += 2;
        12
    }
}

pub fn call_imm16(cpu: &mut Cpu) -> u8 {
    cpu.push_stack(cpu.registers.pc + 2);
    cpu.registers.pc = cpu.fetch_word();
    24
}

pub fn rst_tgt3(cpu: &mut Cpu) -> u8 {
    cpu.push_stack(cpu.registers.pc);
    let target = cpu.current_opcode & 0b0011_1000;
    cpu.registers.pc = target as u16;
    16
}
