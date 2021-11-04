use bit_field::BitField;

fn decode(esr: u64) -> String {
    let res0 = esr.get_bits(37..64);
    let iss2 = esr.get_bits(32..37);
    let ec = esr.get_bits(26..32);
    let il = esr.get_bit(25);
    let iss = esr.get_bits(0..25);
    if res0 != 0 {
        return format!("Invalid ESR, res0 is {}", res0);
    }
    let class = match ec {
        0b000000 => "Unknown reason",
        0b000001 => "Wrapped WF* instruction execution",
        0b000011 => "Trapped MCR or MRC access with coproc=0b1111",
        0b000100 => "Trapped MCRR or MRRC access with coproc=0b1111",
        0b000101 => "Trapped MCR or MRC access with coproc=0b1110",
        0b000110 => "Trapped LDC or STC access",
        0b000111 => "Trapped access to SVE, Advanced SIMD or floating point",
        0b001010 => "Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction",
        0b001100 => "Trapped MRRC access with (coproc==0b1110)",
        0b001101 => "Branch Target Exception",
        0b001110 => "Illegal Execution state",
        0b010001 => "SVC instruction execution in AArch32 state",
        0b010101 => "SVC instruction execution in AArch64 state",
        0b011000 => "Trapped MSR, MRS or System instruction execution in AArch64 state, that is not reported using EC 0b000000, 0b000001, or 0b000111",
        0b011001 => "Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, CPTR_EL2.TZ, or CPTR_EL3.EZ, that is not reported using EC 0b000000",
        0b011100 => "Exception from a Pointer Authentication instruction authentication failure",
        0b100000 => "Instruction Abort from a lower Exception level",
        0b100001 => "Instruction Abort taken without a change in Exception level",
        0b100010 => "PC alignment fault exception",
        0b100100 => "Data Abort from a lower Exception level",
        0b100101 => "Data Abort taken without a change in Exception level",
        0b100110 => "SP alignment fault exception",
        0b101000 => "Trapped floating-point exception taken from AArch32 state",
        0b101100 => "Trapped floating-point exception taken from AArch64 state",
        0b101111 => "SError interrupt",
        0b110000 => "Breakpoint exception from a lower Exception level",
        0b110001 => "Breakpoint exception taken without a change in Exception level",
        0b110010 => "Software Step exception from a lower Exception level",
        0b110011 => "Software Step exception taken without a change in Exception level",
        0b110100 => "Watchpoint exception from a lower Exception level",
        0b110101 => "Watchpoint exception taken without a change in Exception level",
        0b111000 => "BKPT instruction execution in AArch32 state",
        0b111100 => "BRK instruction execution in AArch64 state",
        _ => "Invalid EC",
    };
    format!("EC:{:#08b} '{}', IL:{}, ISS:{:#x}", ec, class, il, iss)
}

fn main() {
    let esr = 2516582480;
    println!("{:#034x}: {}", esr, decode(esr));
}
