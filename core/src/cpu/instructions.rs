use std::fmt;

#[derive(Debug, PartialEq)]
pub enum R8 {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    HLMem = 0b110,
    A = 0b111,
}

impl R8 {
    pub fn get_register(value: u8) -> R8 {
        match value {
            0b000 => R8::B,
            0b001 => R8::C,
            0b010 => R8::D,
            0b011 => R8::E,
            0b100 => R8::H,
            0b101 => R8::L,
            0b110 => R8::HLMem,
            0b111 => R8::A,
            _ => panic!("Invalid value was passed"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16 {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl R16 {
    pub fn get_register(value: u8) -> R16 {
        match value {
            0b00 => R16::BC,
            0b01 => R16::DE,
            0b10 => R16::HL,
            0b11 => R16::SP,
            _ => panic!("Invalid value was passed"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16Stack {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    AF = 0b11,
}

impl R16Stack {
    pub fn get_register(value: u8) -> R16Stack {
        match value {
            0b00 => R16Stack::BC,
            0b01 => R16Stack::DE,
            0b10 => R16Stack::HL,
            0b11 => R16Stack::AF,
            _ => panic!("Invalid value was passed"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16Memory {
    BC = 0b00,
    DE = 0b01,
    HLI = 0b10,
    HLD = 0b11,
}

impl R16Memory {
    pub fn get_register(value: u8) -> R16Memory {
        match value {
            0b00 => R16Memory::BC,
            0b01 => R16Memory::DE,
            0b10 => R16Memory::HLI,
            0b11 => R16Memory::HLD,
            _ => panic!("Invalid value was passed"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    NZ = 0b00,
    Z = 0b01,
    NC = 0b10,
    C = 0b11,
}

impl Condition {
    pub fn get_condtion(value: u8) -> Condition {
        match value {
            0b000 => Condition::NZ,
            0b001 => Condition::Z,
            0b010 => Condition::NC,
            0b011 => Condition::C,
            _ => panic!("Invalid value was passed"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    None,
    Nop,
    LdR16Imm16,
    LdR16MemA,
    LdAR16Mem,
    LdImm16Sp,
    IncR16,
    DecR16,
    AddHlR16,
    IncR8,
    DecR8,
    LdR8Imm8,
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    JrImm8,
    JrCondImm8,
    Stop,
    LdR8R8,
    Halt,
    AddAR8,
    AdcAR8,
    SubAR8,
    SbcAR8,
    AndAR8,
    XorAR8,
    OrAR8,
    CpAR8,
    AddAImm8,
    AdcAImm8,
    SubAImm8,
    SbcAImm8,
    AndAImm8,
    XorAImm8,
    OrAImm8,
    CpAImm8,
    RetCond,
    Ret,
    Reti,
    JpCondImm16,
    JpImm16,
    JpHl,
    CallCondImm16,
    CallImm16,
    RstTgt3,
    PopR16Stk,
    PushR16Stk,
    Prefix,
    LdhCMemA,
    LdhImm8MemA,
    LdImm16MemA,
    LdhACMem,
    LdhAImm8Mem,
    LdAImm16Mem,
    AddSpImm8,
    LdHlSpPlusImm8,
    LdSpHl,
    Di,
    Ei,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::None => write!(f, "NONE"),
            Instruction::Nop => write!(f, "NOP"),
            Instruction::LdR16Imm16
            | Instruction::LdR16MemA
            | Instruction::LdAR16Mem
            | Instruction::LdImm16Sp
            | Instruction::LdR8Imm8
            | Instruction::LdR8R8
            | Instruction::LdhCMemA
            | Instruction::LdhImm8MemA
            | Instruction::LdImm16MemA
            | Instruction::LdhACMem
            | Instruction::LdhAImm8Mem
            | Instruction::LdAImm16Mem
            | Instruction::LdHlSpPlusImm8
            | Instruction::LdSpHl => write!(f, "LD"),
            Instruction::IncR16 | Instruction::IncR8 => write!(f, "INC"),
            Instruction::DecR16 | Instruction::DecR8 => write!(f, "DEC"),
            Instruction::Rlca => write!(f, "RLCA"),
            Instruction::Rrca => write!(f, "RRCA"),
            Instruction::Rla => write!(f, "RLA"),
            Instruction::Rra => write!(f, "RRA"),
            Instruction::Daa => write!(f, "DAA"),
            Instruction::Cpl => write!(f, "CPL"),
            Instruction::Scf => write!(f, "SCF"),
            Instruction::Ccf => write!(f, "CCF"),
            Instruction::JrImm8 | Instruction::JrCondImm8 => write!(f, "JR"),
            Instruction::Stop => write!(f, "STOP"),
            Instruction::Halt => write!(f, "HALT"),
            Instruction::AddHlR16 | Instruction::AddAR8 | Instruction::AddSpImm8 | Instruction::AddAImm8 => write!(f, "ADD"),
            Instruction::AdcAR8 | Instruction::AdcAImm8 => write!(f, "ADC"),
            Instruction::SubAR8 | Instruction::SubAImm8 => write!(f, "SUB"),
            Instruction::SbcAR8 | Instruction::SbcAImm8 => write!(f, "SBC"),
            Instruction::AndAR8 | Instruction::AndAImm8 => write!(f, "AND"),
            Instruction::XorAR8 | Instruction::XorAImm8 => write!(f, "XOR"),
            Instruction::OrAR8 | Instruction::OrAImm8 => write!(f, "OR"),
            Instruction::CpAR8 | Instruction::CpAImm8 => write!(f, "CP"),
            Instruction::RetCond | Instruction::Ret => write!(f, "RET"),
            Instruction::Reti => write!(f, "RETI"),
            Instruction::JpCondImm16 | Instruction::JpImm16 | Instruction::JpHl => write!(f, "JP"),
            Instruction::CallCondImm16 | Instruction::CallImm16 => write!(f, "CALL"),
            Instruction::RstTgt3 => write!(f, "RST"),
            Instruction::PopR16Stk => write!(f, "POP"),
            Instruction::PushR16Stk => write!(f, "PUSH"),
            Instruction::Prefix => write!(f, "CB"),
            Instruction::Di => write!(f, "DI"),
            Instruction::Ei => write!(f, "EI"),
            // Instruction::RlcR8 => write!(f, "RLC"),
            // Instruction::RrcR8 => write!(f, "RRC"),
            // Instruction::RlR8 => write!(f, "RL"),
            // Instruction::RrR8 => write!(f, "RR"),
            // Instruction::SlaR8 => write!(f, "SLA"),
            // Instruction::SraR8 => write!(f, "SRA"),
            // Instruction::SwapR8 => write!(f, "SWAP"),
            // Instruction::SrlR8 => write!(f, "SRL"),
            // Instruction::BitB3R8 => write!(f, "BIT"),
            // Instruction::ResB3R8 => write!(f, "RES"),
            // Instruction::SetB3R8 => write!(f, "SET"),
        }
    }
}

pub fn get_instruction_by_opcode(opcode: u8) -> Instruction {
    match opcode {
        0x00 => Instruction::Nop,
        0x01 | 0x11 | 0x21 | 0x31 => Instruction::LdR16Imm16,
        0x02 | 0x12 | 0x22 | 0x32 => Instruction::LdR16MemA,
        0x03 | 0x13 | 0x23 | 0x33 => Instruction::IncR16,
        0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => Instruction::IncR8,
        0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => Instruction::DecR8,
        0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => Instruction::LdR8Imm8,
        0x10 => Instruction::Stop,
        0x07 => Instruction::Rlca,
        0x17 => Instruction::Rrca,
        0x27 => Instruction::Daa,
        0x37 => Instruction::Scf,
        0x08 => Instruction::LdImm16Sp,
        0x18 => Instruction::JrImm8,
        0x20 | 0x28 | 0x30 | 0x38 => Instruction::JrCondImm8,
        0x09 | 0x19 | 0x29 | 0x39 => Instruction::AddHlR16,
        0x0A | 0x1A | 0x2A | 0x3A => Instruction::LdAR16Mem,
        0x0B | 0x1B | 0x2B | 0x3B => Instruction::DecR16,
        0x0F => Instruction::Rla,
        0x1F => Instruction::Rra,
        0x2F => Instruction::Cpl,
        0x3F => Instruction::Ccf,
        0x40..=0x75 | 0x77..=0x7F => Instruction::LdR8R8,
        0x76 => Instruction::Halt,
        0x80..=0x87 => Instruction::AddAR8,
        0x88..=0x8F => Instruction::AdcAR8,
        0x90..=0x97 => Instruction::SubAR8,
        0x98..=0x9F => Instruction::SbcAR8,
        0xA0..=0xA7 => Instruction::AndAR8,
        0xA8..=0xAF => Instruction::XorAR8,
        0xB0..=0xB7 => Instruction::OrAR8,
        0xB8..=0xBF => Instruction::CpAR8,
        0xC0 | 0xC8 | 0xD0 | 0xD8 => Instruction::RetCond,
        0xE0 => Instruction::LdhImm8MemA,
        0xF0 => Instruction::LdhAImm8Mem,
        0xC1 | 0xD1 | 0xE1 | 0xF1 => Instruction::PopR16Stk,
        0xC2 | 0xCA | 0xD2 | 0xDA => Instruction::JpCondImm16,
        0xE2 => Instruction::LdhCMemA,
        0xF2 => Instruction::LdhACMem,
        0xC3 => Instruction::JpImm16,
        0xF3 => Instruction::Di,
        0xC4 | 0xCC | 0xD4 | 0xDC => Instruction::CallCondImm16,
        0xC5 | 0xD5 | 0xE5 | 0xF5 => Instruction::PushR16Stk,
        0xC6 => Instruction::AddAImm8,
        0xD6 => Instruction::SubAImm8,
        0xE6 => Instruction::AndAImm8,
        0xF6 => Instruction::OrAImm8,
        0xC7 | 0xD7 | 0xE7 | 0xF7 | 0xCF | 0xDF | 0xEF | 0xFF => Instruction::RstTgt3,
        0xE8 => Instruction::AddSpImm8,
        0xF8 => Instruction::LdHlSpPlusImm8,
        0xC9 => Instruction::Ret,
        0xD9 => Instruction::Reti,
        0xE9 => Instruction::JpHl,
        0xF9 => Instruction::LdSpHl,
        0xEA => Instruction::LdImm16MemA,
        0xFA => Instruction::LdAImm16Mem,
        0xCB => Instruction::Prefix,
        0xFB => Instruction::Ei,
        0xCD => Instruction::CallImm16,
        0xCE => Instruction::AdcAImm8,
        0xDE => Instruction::SbcAImm8,
        0xEE => Instruction::XorAImm8,
        0xFE => Instruction::CpAImm8,
        code => panic!("Code {:#04X} not implemented", code),
    }
}

pub fn instruction_name(instruction: &Instruction) -> String {
    instruction.to_string()
}
