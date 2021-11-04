use bit_field::BitField;
use std::fmt::{self, Debug, Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
enum DecodeError {
    #[error("Invalid ESR, res0 is {res0}")]
    InvalidRes0 { res0: u64 },
    #[error("Invalid EC {ec}")]
    InvalidEc { ec: u64 },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SyndromeAccessSize {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
    Doubleword = 0b11,
}

impl Display for SyndromeAccessSize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::Byte => "byte",
            Self::Halfword => "halfword",
            Self::Word => "word",
            Self::Doubleword => "doubleword",
        };
        write!(f, "{}", s)
    }
}

fn decode_iss_data_abort(iss: u64) -> String {
    let isv = iss.get_bit(24);
    let sas = match iss.get_bits(22..24) {
        0b00 => SyndromeAccessSize::Byte,
        0b01 => SyndromeAccessSize::Halfword,
        0b10 => SyndromeAccessSize::Word,
        0b11 => SyndromeAccessSize::Doubleword,
        _ => unreachable!(),
    };
    let sse = iss.get_bit(21);
    let src = iss.get_bits(16..21);
    let sf = iss.get_bit(15);
    let ar = iss.get_bit(14);
    let vncr = iss.get_bit(13);
    let fnv = iss.get_bit(10);
    let ea = iss.get_bit(9);
    let cm = iss.get_bit(8);
    let s1ptw = iss.get_bit(7);
    let wnr = iss.get_bit(6);
    let dfsc = iss.get_bits(0..6);
    format!("ISV:{}, SAS:{}", isv, sas)
}

fn decode(esr: u64) -> Result<String, DecodeError> {
    let res0 = esr.get_bits(37..64);
    let iss2 = esr.get_bits(32..37);
    let ec = esr.get_bits(26..32);
    let il = esr.get_bit(25);
    let iss = esr.get_bits(0..25);
    if res0 != 0 {
        return Err(DecodeError::InvalidRes0 { res0 });
    }
    let (class, iss_description) = match ec {
        0b000000 => ("Unknown reason", None),
        0b000001 => ("Wrapped WF* instruction execution", None),
        0b000011 => ("Trapped MCR or MRC access with coproc=0b1111", None),
        0b000100 => ("Trapped MCRR or MRRC access with coproc=0b1111", None),
        0b000101 => ("Trapped MCR or MRC access with coproc=0b1110", None),
        0b000110 => ("Trapped LDC or STC access", None),
        0b000111 => ("Trapped access to SVE, Advanced SIMD or floating point", None),
        0b001010 => ("Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction", None),
        0b001100 => ("Trapped MRRC access with (coproc==0b1110)", None),
        0b001101 => ("Branch Target Exception", None),
        0b001110 => ("Illegal Execution state", None),
        0b010001 => ("SVC instruction execution in AArch32 state", None),
        0b010101 => ("SVC instruction execution in AArch64 state", None),
        0b011000 => ("Trapped MSR, MRS or System instruction execution in AArch64 state", None),
        0b011001 => ("Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, CPTR_EL2.TZ, or CPTR_EL3.EZ", None),
        0b011100 => ("Exception from a Pointer Authentication instruction authentication failure", None),
        0b100000 => ("Instruction Abort from a lower Exception level", None),
        0b100001 => ("Instruction Abort taken without a change in Exception level", None),
        0b100010 => ("PC alignment fault exception", None),
        0b100100 => ("Data Abort from a lower Exception level", Some(decode_iss_data_abort(iss))),
        0b100101 => ("Data Abort taken without a change in Exception level", Some(decode_iss_data_abort(iss))),
        0b100110 => ("SP alignment fault exception", None),
        0b101000 => ("Trapped floating-point exception taken from AArch32 state", None),
        0b101100 => ("Trapped floating-point exception taken from AArch64 state", None),
        0b101111 => ("SError interrupt", None),
        0b110000 => ("Breakpoint exception from a lower Exception level", None),
        0b110001 => ("Breakpoint exception taken without a change in Exception level", None),
        0b110010 => ("Software Step exception from a lower Exception level", None),
        0b110011 => ("Software Step exception taken without a change in Exception level", None),
        0b110100 => ("Watchpoint exception from a lower Exception level", None),
        0b110101 => ("Watchpoint exception taken without a change in Exception level", None),
        0b111000 => ("BKPT instruction execution in AArch32 state", None),
        0b111100 => ("BRK instruction execution in AArch64 state", None),
        _ => return Err(DecodeError::InvalidEc { ec }),
    };
    if let Some(iss_description) = iss_description {
        Ok(format!(
            "EC:{:#08b} '{}', IL:{}, ISS:{:#x} ({}), ISS2:{:#x}",
            ec, class, il, iss, iss_description, iss2
        ))
    } else {
        Ok(format!(
            "EC:{:#08b} '{}', IL:{}, ISS:{:#x}, ISS2:{:#x}",
            ec, class, il, iss, iss2
        ))
    }
}

fn main() {
    let esr = 2516582480;
    println!("{:#034x}: {}", esr, decode(esr).unwrap());
}
