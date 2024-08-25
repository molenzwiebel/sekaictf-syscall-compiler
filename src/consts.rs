use iced_x86::code_asm::{AsmRegister16, AsmRegister32, AsmRegister64};
use regalloc2::PReg;

pub const DEBUG: bool = false;
pub const REAL_JUMPS: bool = false;
pub const SHUFFLE_BLOCKS: bool = true;
pub const BUILD_AS_EXE: bool = true;

// Physical registers
pub const RAX: PReg = PReg::new(0, regalloc2::RegClass::Int);
pub const RBX: PReg = PReg::new(1, regalloc2::RegClass::Int);
pub const RCX: PReg = PReg::new(2, regalloc2::RegClass::Int);
pub const RDX: PReg = PReg::new(3, regalloc2::RegClass::Int);
pub const RDI: PReg = PReg::new(4, regalloc2::RegClass::Int);
pub const RSI: PReg = PReg::new(5, regalloc2::RegClass::Int);
pub const RBP: PReg = PReg::new(6, regalloc2::RegClass::Int);
pub const RSP: PReg = PReg::new(7, regalloc2::RegClass::Int);
pub const R8: PReg = PReg::new(8, regalloc2::RegClass::Int);
pub const R9: PReg = PReg::new(9, regalloc2::RegClass::Int);
pub const R10: PReg = PReg::new(10, regalloc2::RegClass::Int);
pub const R11: PReg = PReg::new(11, regalloc2::RegClass::Int);
pub const R12: PReg = PReg::new(12, regalloc2::RegClass::Int);
pub const R13: PReg = PReg::new(13, regalloc2::RegClass::Int);
pub const R14: PReg = PReg::new(14, regalloc2::RegClass::Int);
pub const R15: PReg = PReg::new(15, regalloc2::RegClass::Int);

pub const ARGS: [PReg; 4] = [RCX, RDX, R8, R9];

pub fn preg_to_name(reg: PReg) -> &'static str {
    if reg == RAX {
        return "RAX";
    } else if reg == RBX {
        return "RBX";
    } else if reg == RCX {
        return "RCX";
    } else if reg == RDX {
        return "RDX";
    } else if reg == RDI {
        return "RDI";
    } else if reg == RSI {
        return "RSI";
    } else if reg == RBP {
        return "RBP";
    } else if reg == RSP {
        return "RSP";
    } else if reg == R8 {
        return "R8";
    } else if reg == R9 {
        return "R9";
    } else if reg == R10 {
        return "R10";
    } else if reg == R11 {
        return "R11";
    } else if reg == R12 {
        return "R12";
    } else if reg == R13 {
        return "R13";
    } else if reg == R14 {
        return "R14";
    } else if reg == R15 {
        return "R15";
    }

    unreachable!("where'd this preg come from")
}

pub fn preg_to_iced_reg(reg: PReg) -> AsmRegister64 {
    use iced_x86::code_asm::*;
    if reg == RAX {
        return rax;
    } else if reg == RBX {
        return rbx;
    } else if reg == RCX {
        return rcx;
    } else if reg == RDX {
        return rdx;
    } else if reg == RDI {
        return rdi;
    } else if reg == RSI {
        return rsi;
    } else if reg == RBP {
        return rbp;
    } else if reg == RSP {
        return rsp;
    } else if reg == R8 {
        return r8;
    } else if reg == R9 {
        return r9;
    } else if reg == R10 {
        return r10;
    } else if reg == R11 {
        return r11;
    } else if reg == R12 {
        return r12;
    } else if reg == R13 {
        return r13;
    } else if reg == R14 {
        return r14;
    } else if reg == R15 {
        return r15;
    }
    unreachable!("where'd this preg come from")
}

pub fn iced_reg_to_16_bit(reg: AsmRegister64) -> AsmRegister16 {
    use iced_x86::code_asm::*;
    match reg {
        rax => ax,
        rbx => bx,
        rcx => cx,
        rdx => dx,
        rdi => di,
        rsi => si,
        rbp => bp,
        r8 => r8w,
        r9 => r9w,
        r10 => r10w,
        r11 => r11w,
        r12 => r12w,
        r13 => r13w,
        r14 => r14w,
        r15 => r15w,
        _ => unreachable!("where'd this reg come from"),
    }
}

pub fn iced_reg_to_16_bit_upper(reg: AsmRegister64) -> AsmRegister16 {
    use iced_x86::code_asm::*;
    match reg {
        rax => ax,
        rbx => bx,
        rcx => cx,
        rdx => dx,
        rdi => di,
        rsi => si,
        rbp => bp,
        r8 => r8w,
        r9 => r9w,
        r10 => r10w,
        r11 => r11w,
        r12 => r12w,
        r13 => r13w,
        r14 => r14w,
        r15 => r15w,
        _ => unreachable!("where'd this reg come from"),
    }
}

pub fn iced_reg_to_32_bit(reg: AsmRegister64) -> AsmRegister32 {
    use iced_x86::code_asm::*;
    match reg {
        rax => eax,
        rbx => ebx,
        rcx => ecx,
        rdx => edx,
        rdi => edi,
        rsi => esi,
        rbp => ebp,
        r8 => r8d,
        r9 => r9d,
        r10 => r10d,
        r11 => r11d,
        r12 => r12d,
        r13 => r13d,
        r14 => r14d,
        r15 => r15d,
        _ => unreachable!("where'd this reg come from"),
    }
}

pub const FILE_ALIGN: u32 = 0x1000;
pub const SECT_ALIGN: u32 = 0x1000;

pub const BASE_ADDRESS_EXECUTABLE: u32 = 0x400000;
pub const BASE_ADDRESS_TEXT: u32 = BASE_ADDRESS_EXECUTABLE + 0x1000;
pub const BASE_ADDRESS_DATA: u32 = 0xa00000;

pub const TEXT_SIZE: u32 = BASE_ADDRESS_DATA - BASE_ADDRESS_TEXT;

pub const TEXT_SECTION_RVA: u32 = BASE_ADDRESS_TEXT - BASE_ADDRESS_EXECUTABLE;
pub const DATA_SECTION_RVA: u32 = BASE_ADDRESS_DATA - BASE_ADDRESS_EXECUTABLE;
