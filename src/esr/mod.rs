mod abort;
mod breakpoint;
mod bti;
mod common;
mod fp;
mod hvc;
mod ld64b;
mod ldc;
mod mcr;
mod msr;
mod pauth;
mod serror;
mod sve;
#[cfg(test)]
mod tests;
mod wf;

use super::{DecodeError, FieldInfo};
use abort::{decode_iss_data_abort, decode_iss_instruction_abort};
use breakpoint::{
    decode_iss_breakpoint, decode_iss_breakpoint_vector_catch, decode_iss_software_step,
    decode_iss_watchpoint,
};
use bti::decode_iss_bti;
use fp::decode_iss_fp;
use hvc::decode_iss_hvc;
use ld64b::decode_iss_ld64b;
use ldc::decode_iss_ldc;
use mcr::{decode_iss_mcr, decode_iss_mcrr};
use msr::decode_iss_msr;
use pauth::decode_iss_pauth;
use serror::decode_iss_serror;
use sve::decode_iss_sve;
use wf::decode_iss_wf;

fn decode_iss_res0(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 0, 25)
        .check_res0()?
        .with_description("ISS is RES0".to_string());
    Ok(vec![res0])
}

/// Decodes the given Exception Syndrome Register value, or returns an error if it is not valid.
pub fn decode(esr: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(esr, "RES0", Some("Reserved"), 37, 64).check_res0()?;
    let iss2 = FieldInfo::get(esr, "ISS2", None, 32, 37);
    let ec = FieldInfo::get(esr, "EC", Some("Exception Class"), 26, 32);
    let il =
        FieldInfo::get_bit(esr, "IL", Some("Instruction Length"), 25).describe_bit(describe_il);
    let iss = FieldInfo::get(esr, "ISS", Some("Instruction Specific Syndrome"), 0, 25);
    let (class, iss_subfields, iss_description) = match ec.value {
        0b000000 => ("Unknown reason", decode_iss_res0(iss.value)?, None),
        0b000001 => (
            "Wrapped WF* instruction execution",
            decode_iss_wf(iss.value)?,
            None,
        ),
        0b000011 => (
            "Trapped MCR or MRC access with coproc=0b1111",
            decode_iss_mcr(iss.value)?,
            None,
        ),
        0b000100 => (
            "Trapped MCRR or MRRC access with coproc=0b1111",
            decode_iss_mcrr(iss.value)?,
            None,
        ),
        0b000101 => (
            "Trapped MCR or MRC access with coproc=0b1110",
            decode_iss_mcr(iss.value)?,
            None,
        ),
        0b000110 => (
            "Trapped LDC or STC access",
            decode_iss_ldc(iss.value)?,
            None,
        ),
        0b000111 => (
            "Trapped access to SVE, Advanced SIMD or floating point",
            decode_iss_sve(iss.value)?,
            None,
        ),
        0b001010 => (
            "Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction",
            decode_iss_ld64b(iss.value)?,
            None,
        ),
        0b001100 => (
            "Trapped MRRC access with (coproc==0b1110)",
            decode_iss_mcrr(iss.value)?,
            None,
        ),
        0b001101 => ("Branch Target Exception", decode_iss_bti(iss.value)?, None),
        0b001110 => ("Illegal Execution state", decode_iss_res0(iss.value)?, None),
        0b010001 => (
            "SVC instruction execution in AArch32 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b010101 => (
            "SVC instruction execution in AArch64 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b010110 => (
            "HVC instruction execution in AArch64 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b010111 => (
            "SMC instruction execution in AArch64 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b011000 => {
            let (subfields, description) = decode_iss_msr(iss.value)?;
            (
                "Trapped MSR, MRS or System instruction execution in AArch64 state",
                subfields,
                description,
            )
        }
        0b011001 => (
            "Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, \
                 CPTR_EL2.TZ, or CPTR_EL3.EZ",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b011100 => (
            "Exception from a Pointer Authentication instruction authentication failure",
            decode_iss_pauth(iss.value)?,
            None,
        ),
        0b100000 => (
            "Instruction Abort from a lower Exception level",
            decode_iss_instruction_abort(iss.value)?,
            None,
        ),
        0b100001 => (
            "Instruction Abort taken without a change in Exception level",
            decode_iss_instruction_abort(iss.value)?,
            None,
        ),
        0b100010 => (
            "PC alignment fault exception",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b100100 => (
            "Data Abort from a lower Exception level",
            decode_iss_data_abort(iss.value)?,
            None,
        ),
        0b100101 => (
            "Data Abort taken without a change in Exception level",
            decode_iss_data_abort(iss.value)?,
            None,
        ),
        0b100110 => (
            "SP alignment fault exception",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b101000 => (
            "Trapped floating-point exception taken from AArch32 state",
            decode_iss_fp(iss.value)?,
            None,
        ),
        0b101100 => (
            "Trapped floating-point exception taken from AArch64 state",
            decode_iss_fp(iss.value)?,
            None,
        ),
        0b101111 => ("SError interrupt", decode_iss_serror(iss.value)?, None),
        0b110000 => (
            "Breakpoint exception from a lower Exception level",
            decode_iss_breakpoint_vector_catch(iss.value)?,
            None,
        ),
        0b110001 => (
            "Breakpoint exception taken without a change in Exception level",
            decode_iss_breakpoint_vector_catch(iss.value)?,
            None,
        ),
        0b110010 => (
            "Software Step exception from a lower Exception level",
            decode_iss_software_step(iss.value)?,
            None,
        ),
        0b110011 => (
            "Software Step exception taken without a change in Exception level",
            decode_iss_software_step(iss.value)?,
            None,
        ),
        0b110100 => (
            "Watchpoint exception from a lower Exception level",
            decode_iss_watchpoint(iss.value)?,
            None,
        ),
        0b110101 => (
            "Watchpoint exception taken without a change in Exception level",
            decode_iss_watchpoint(iss.value)?,
            None,
        ),
        0b111000 => (
            "BKPT instruction execution in AArch32 state",
            decode_iss_breakpoint(iss.value)?,
            None,
        ),
        0b111100 => (
            "BRK instruction execution in AArch64 state",
            decode_iss_breakpoint(iss.value)?,
            None,
        ),
        _ => return Err(DecodeError::InvalidEc { ec: ec.value }),
    };
    let iss = FieldInfo {
        description: iss_description,
        subfields: iss_subfields,
        ..iss
    };
    let ec = ec.with_description(class.to_string());
    Ok(vec![res0, iss2, ec, il, iss])
}

fn describe_il(il: bool) -> &'static str {
    if il {
        "32-bit instruction trapped"
    } else {
        "16-bit instruction trapped"
    }
}
