use std::fmt;

use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Register,
    RegisterAddress,
    RegisterToRegister,
    RegisterToRegisterAddress,
    RegisterToU8Address,
    RegisterToU16Address,
    RegisterAddressToRegister,
    RegisterPlusI8ToRegister,
    I8,
    U8,
    U8ToRegister,
    U8ToRegisterAddress,
    U8AddressToRegister,
    U16,
    U16ToRegister,
    U16AddressToRegister,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum RegisterType {
    None = 0,
    A = 1,
    F = 2,
    B = 3,
    C = 4,
    D = 5,
    E = 6,
    H = 7,
    L = 8,
    AF = 9,
    BC = 10,
    DE = 11,
    HL = 12,
    HLI = 13,
    HLD = 14,
    SP = 15,
    PC = 16,
}

#[derive(Clone, Copy, Debug)]
pub enum ConditionType {
    None,
    NZ,
    Z,
    NC,
    C,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InstructionType {
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    DI,
    EI,
    RST,
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub addressing_mode: AddressingMode,
    pub register_1: RegisterType,
    pub register_2: RegisterType,
    pub condition: ConditionType,
    pub parameter: Option<u8>,
}

impl Instruction {
    pub fn new(
        instruction_type: InstructionType,
        addressing_mode: AddressingMode,
        register_1: RegisterType,
        register_2: RegisterType,
        condition: ConditionType,
        parameter: Option<u8>,
    ) -> Self {
        Instruction {
            instruction_type,
            addressing_mode,
            register_1,
            register_2,
            condition,
            parameter,
        }
    }
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            instruction_type: InstructionType::NONE,
            addressing_mode: AddressingMode::Implied,
            register_1: RegisterType::None,
            register_2: RegisterType::None,
            condition: ConditionType::None,
            parameter: None,
        }
    }
}

pub fn get_instruction_by_opcode(opcode: u8) -> &'static Instruction {
    &UNPREFIXED_INSTRUCTIONS[opcode as usize]
}

pub fn instruction_name(instruction_type: &InstructionType) -> String {
    instruction_type.to_string()
}

lazy_static! {
    static ref UNPREFIXED_INSTRUCTIONS:  Vec<Instruction> = vec![
        Instruction::new(InstructionType::NOP, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U16ToRegister, RegisterType::BC, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::BC, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::BC, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::B, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::B, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::B, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RLCA, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToU16Address, RegisterType::None, RegisterType::SP, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::HL, RegisterType::BC, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::A, RegisterType::BC, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::BC, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::C, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::C, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::C, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RRCA, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),

        //0x1X
        Instruction::new(InstructionType::STOP, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U16ToRegister, RegisterType::DE, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::DE, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::DE, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::D, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::D, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::D, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RLA, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JR, AddressingMode::I8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::HL, RegisterType::DE, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::DE, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::DE, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::E, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::E, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::E, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RRA, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),

        //0x2X
        Instruction::new(InstructionType::JR, AddressingMode::I8, RegisterType::None, RegisterType::None, ConditionType::NZ, None),
        Instruction::new(InstructionType::LD, AddressingMode::U16ToRegister, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::HLI, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::H, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::H, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::H, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DAA, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JR, AddressingMode::I8, RegisterType::None, RegisterType::None, ConditionType::Z, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::HL, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HLI, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::L, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::L, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::L, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CPL, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),

        //0x3X
        Instruction::new(InstructionType::JR, AddressingMode::I8, RegisterType::None, RegisterType::None, ConditionType::NC, None),
        Instruction::new(InstructionType::LD, AddressingMode::U16ToRegister, RegisterType::SP, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::HLD, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::SP, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::RegisterAddress, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::RegisterAddress, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegisterAddress, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::SCF, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JR, AddressingMode::I8, RegisterType::None, RegisterType::None, ConditionType::C, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::HL, RegisterType::SP, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HLD, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::SP, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::INC, AddressingMode::Register, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::DEC, AddressingMode::Register, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U8ToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CCF, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),

        //0x4X
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::B, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::B, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::C, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::C, RegisterType::A, ConditionType::None, None),

        //0x5X
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::D, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::D, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::E, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::E, RegisterType::A, ConditionType::None, None),

        //0x6X
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::H, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::H, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::L, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::L, RegisterType::A, ConditionType::None, None),

        //0x7X
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::HALT, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress,  RegisterType::HL, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister,  RegisterType::A, RegisterType::A, ConditionType::None, None),

        //0x8X
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),

        //0x9X
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),

        //0xAX
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),

        //0xBX
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::B, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::D, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::E, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::H, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::L, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::RegisterToRegister, RegisterType::A, RegisterType::A, ConditionType::None, None),

        //0xCX
        Instruction::new(InstructionType::RET, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::NZ, None),
        Instruction::new(InstructionType::POP, AddressingMode::Register, RegisterType::BC, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JP, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::NZ, None),
        Instruction::new(InstructionType::JP, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CALL, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::NZ, None),
        Instruction::new(InstructionType::PUSH, AddressingMode::Register, RegisterType::BC, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::ADD, AddressingMode::U8AddressToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x00)),
        Instruction::new(InstructionType::RET, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::Z, None),
        Instruction::new(InstructionType::RET, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JP, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::Z, None),
        Instruction::new(InstructionType::CB, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CALL, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::Z, None),
        Instruction::new(InstructionType::CALL, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::ADC, AddressingMode::U8ToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x08)),

        //0xDX
        Instruction::new(InstructionType::RET, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::NC, None),
        Instruction::new(InstructionType::POP, AddressingMode::Register, RegisterType::DE, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JP, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::NC, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CALL, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::NC, None),
        Instruction::new(InstructionType::PUSH, AddressingMode::Register, RegisterType::DE, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::SUB, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x10)),
        Instruction::new(InstructionType::RET, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::C, None),
        Instruction::new(InstructionType::RETI, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JP, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::C, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CALL, AddressingMode::U16, RegisterType::None, RegisterType::None, ConditionType::C, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::SBC, AddressingMode::U8ToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x18)),

        //0xEX
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToU8Address, RegisterType::None, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::POP, AddressingMode::Register, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegisterAddress, RegisterType::None, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::PUSH, AddressingMode::Register, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::AND, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x20)),
        Instruction::new(InstructionType::ADD, AddressingMode::U8ToRegister, RegisterType::SP, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::JP, AddressingMode::RegisterAddress, RegisterType::HL, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToU16Address, RegisterType::None, RegisterType::A, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::XOR, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x28)),

        //0xFX
        Instruction::new(InstructionType::LD, AddressingMode::U8AddressToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::POP, AddressingMode::Register, RegisterType::AF, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterAddressToRegister, RegisterType::A, RegisterType::C, ConditionType::None, None),
        Instruction::new(InstructionType::DI, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::PUSH, AddressingMode::Register, RegisterType::AF, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::OR, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x30)),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterPlusI8ToRegister, RegisterType::HL, RegisterType::SP, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::RegisterToRegister, RegisterType::SP, RegisterType::HL, ConditionType::None, None),
        Instruction::new(InstructionType::LD, AddressingMode::U16AddressToRegister, RegisterType::A, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::EI, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::NONE, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::CP, AddressingMode::U8, RegisterType::None, RegisterType::None, ConditionType::None, None),
        Instruction::new(InstructionType::RST, AddressingMode::Implied, RegisterType::None, RegisterType::None, ConditionType::None, Some(0x38)),
    ];
}
