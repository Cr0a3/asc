use CodeGenLib::IR::Register;

pub fn is_reg(name: &str) -> bool {
    match name {
        "rax" => true,
        "rcx" => true,
        "rdx" => true,
        "rbx" => true,
        "rsi" => true,
        "rdi" => true,
        "rsp" => true,
        "rbp" => true,
        "r8" => true,
        "r9" => true,
        "r10" => true,
        "r11" => true,
        "r12" => true,
        "r13" => true,
        "r14" => true,
        "r15" => true,
        _ => false,
    }
}

pub fn to_reg(name: &str) -> Register {
    match name {
        "rax" => Register::RAX,
        "rcx" => Register::RCX,
        "rdx" => Register::RDX,
        "rbx" => Register::RBX,
        "rsi" => Register::RSI,
        "rdi" => Register::RDI,
        "rsp" => Register::RSP,
        "rbp" => Register::RBP,
        "r8" => Register::R8,
        "r9" => Register::R9,
        "r10" => Register::R10,
        "r11" => Register::R11,
        "r12" => Register::R12,
        "r13" => Register::R13,
        "r14" => Register::R14,
        "r15" => Register::R15,
        _ => Register::None,
    }
}