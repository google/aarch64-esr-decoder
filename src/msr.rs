// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{DecodeError, FieldInfo};

/// Decodes the ISS value for an MSR or MRS instruction.
pub fn decode_iss_msr(iss: u64) -> Result<(Vec<FieldInfo>, Option<String>), DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 22, 25).check_res0()?;
    let op0 = FieldInfo::get(iss, "Op0", None, 20, 22);
    let op2 = FieldInfo::get(iss, "Op2", None, 17, 20);
    let op1 = FieldInfo::get(iss, "Op1", None, 14, 17);
    let crn = FieldInfo::get(iss, "CRn", None, 10, 14);
    let rt = FieldInfo::get(
        iss,
        "Rt",
        Some("General-purpose register number of the trapped instruction"),
        5,
        10,
    );
    let crm = FieldInfo::get(iss, "CRm", None, 1, 5);
    let direction = FieldInfo::get_bit(
        iss,
        "Direction",
        Some("Direction of the trapped instruction"),
        0,
    )
    .describe_bit(describe_direction);

    let name = sysreg_name(op0.value, op1.value, op2.value, crn.value, crm.value);
    let description = if direction.value == 0 {
        format!("MSR {}, x{}", name, rt.value)
    } else {
        format!("MRS x{}, {}", rt.value, name)
    };

    Ok((
        vec![res0, op0, op2, op1, crn, rt, crm, direction],
        Some(description),
    ))
}

fn describe_direction(direction: bool) -> &'static str {
    if direction {
        "Read from system register (MRS)"
    } else {
        "Write to system register (MSR)"
    }
}

fn sysreg_name(op0: u64, op1: u64, op2: u64, crn: u64, crm: u64) -> &'static str {
    match (op0, crn, op1, crm, op2) {
        (3, 1, 0, 0, 1) => "ACTLR_EL1",
        (3, 1, 4, 0, 1) => "ACTLR_EL2",
        (3, 1, 6, 0, 1) => "ACTLR_EL3",
        (3, 0, 1, 0, 7) => "AIDR_EL1",
        (3, 5, 0, 1, 0) => "AFSR0_EL1",
        (3, 5, 4, 1, 0) => "AFSR0_EL2",
        (3, 5, 6, 1, 0) => "AFSR0_EL3",
        (3, 5, 0, 1, 1) => "AFSR1_EL1",
        (3, 5, 4, 1, 1) => "AFSR1_EL2",
        (3, 5, 6, 1, 1) => "AFSR1_EL3",
        (3, 10, 0, 3, 0) => "AMAIR_EL1",
        (3, 10, 4, 3, 0) => "AMAIR_EL2",
        (3, 10, 6, 3, 0) => "AMAIR_EL3",
        (3, 0, 1, 0, 0) => "CCSIDR_EL1",
        (3, 0, 1, 0, 1) => "CLIDR_EL1",
        (3, 1, 0, 0, 2) => "CPACR_EL1",
        (3, 1, 4, 1, 2) => "CPTR_EL2",
        (3, 1, 6, 1, 2) => "CPTR_EL3",
        (3, 0, 2, 0, 0) => "CSSELR_EL1",
        (3, 0, 3, 0, 1) => "CTR_EL0",
        (3, 12, 0, 1, 1) => "DISR_EL1",
        (3, 5, 0, 3, 0) => "ERRIDR_EL1",
        (3, 5, 0, 3, 1) => "ERRSELR_EL1",
        (3, 5, 0, 4, 3) => "ERXADDR_EL1",
        (3, 5, 0, 4, 1) => "ERXCTLR_EL1",
        (3, 5, 0, 4, 0) => "ERXFR_EL1",
        (3, 5, 0, 5, 0) => "ERXMISC0_EL1",
        (3, 5, 0, 5, 1) => "ERXMISC1_EL1",
        (3, 5, 0, 4, 2) => "ERXSTATUS_EL1",
        (3, 5, 0, 2, 0) => "ESR_EL1",
        (3, 5, 4, 2, 0) => "ESR_EL2",
        (3, 5, 6, 2, 0) => "ESR_EL3",
        (3, 1, 4, 1, 7) => "HACR_EL2",
        (3, 1, 4, 1, 0) => "HCR_EL2",
        (3, 0, 0, 1, 3) => "ID_AFR0_EL1",
        (3, 0, 0, 1, 2) => "ID_DFR0_EL1",
        (3, 0, 0, 2, 0) => "ID_ISAR0_EL1",
        (3, 0, 0, 2, 1) => "ID_ISAR1_EL1",
        (3, 0, 0, 2, 2) => "ID_ISAR2_EL1",
        (3, 0, 0, 2, 3) => "ID_ISAR3_EL1",
        (3, 0, 0, 2, 4) => "ID_ISAR4_EL1",
        (3, 0, 0, 2, 5) => "ID_ISAR5_EL1",
        (3, 0, 0, 2, 7) => "ID_ISAR6_EL1",
        (3, 0, 0, 1, 4) => "ID_MMFR0_EL1",
        (3, 0, 0, 1, 5) => "ID_MMFR1_EL1",
        (3, 0, 0, 1, 6) => "ID_MMFR2_EL1",
        (3, 0, 0, 1, 7) => "ID_MMFR3_EL1",
        (3, 0, 0, 2, 6) => "ID_MMFR4_EL1",
        (3, 0, 0, 1, 0) => "ID_PFR0_EL1",
        (3, 0, 0, 1, 1) => "ID_PFR1_EL1",
        (3, 0, 0, 3, 4) => "ID_PFR2_EL1",
        (3, 0, 0, 5, 0) => "ID_AA64DFR0_EL1",
        (3, 0, 0, 6, 0) => "ID_AA64ISAR0_EL1",
        (3, 0, 0, 6, 1) => "ID_AA64ISAR1_EL1",
        (3, 0, 0, 7, 0) => "ID_AA64MMFR0_EL1",
        (3, 0, 0, 7, 1) => "ID_AA64MMFR1_EL1",
        (3, 0, 0, 7, 2) => "ID_AA64MMFR2_EL1",
        (3, 0, 0, 4, 0) => "ID_AA64PFR0_EL1",
        (3, 5, 4, 0, 1) => "IFSR32_EL2",
        (3, 10, 0, 4, 3) => "LORC_EL1",
        (3, 10, 0, 4, 7) => "LORID_EL1",
        (3, 10, 0, 4, 2) => "LORN_EL1",
        (3, 1, 6, 3, 1) => "MDCR_EL3",
        (3, 0, 0, 0, 0) => "MIDR_EL1",
        (3, 0, 0, 0, 5) => "MPIDR_EL1",
        (3, 7, 0, 4, 0) => "PAR_EL1",
        (3, 12, 6, 0, 1) => "RVBAR_EL3",
        (3, 0, 0, 0, 6) => "REVIDR_EL1",
        (3, 1, 0, 0, 0) => "SCTLR_EL1",
        (3, 1, 6, 0, 0) => "SCTLR_EL3",
        (3, 2, 0, 0, 2) => "TCR_EL1",
        (3, 2, 4, 0, 2) => "TCR_EL2",
        (3, 2, 6, 0, 2) => "TCR_EL3",
        (3, 2, 0, 0, 0) => "TTBR0_EL1",
        (3, 2, 4, 0, 0) => "TTBR0_EL2",
        (3, 2, 6, 0, 0) => "TTBR0_EL3",
        (3, 2, 0, 0, 1) => "TTBR1_EL1",
        (3, 2, 4, 0, 1) => "TTBR1_EL2",
        (3, 12, 4, 1, 1) => "VDISR_EL2",
        (3, 5, 4, 2, 3) => "VSESR_EL2",
        (3, 2, 4, 1, 2) => "VTCR_EL2",
        (3, 2, 4, 1, 0) => "VTTBR_EL2",
        (3, 5, 5, 1, 0) => "AFSR0_EL12",
        (3, 5, 5, 1, 1) => "AFSR1_EL12",
        (3, 10, 5, 3, 0) => "AMAIR_EL12",
        (3, 14, 3, 0, 0) => "CNTFRQ_EL0",
        (3, 14, 4, 1, 0) => "CNTHCTL_EL2",
        (3, 14, 4, 2, 1) => "CNTHP_CTL_EL2",
        (3, 14, 4, 2, 2) => "CNTHP_CVAL_EL2",
        (3, 14, 4, 2, 0) => "CNTHP_TVAL_EL2",
        (3, 14, 4, 3, 1) => "CNTHV_CTL_EL2",
        (3, 14, 4, 3, 2) => "CNTHV_CVAL_EL2",
        (3, 14, 4, 3, 0) => "CNTHV_TVAL_EL2",
        (3, 14, 0, 1, 0) => "CNTKCTL_EL1",
        (3, 14, 5, 1, 0) => "CNTKCTL_EL12",
        (3, 14, 3, 2, 1) => "CNTP_CTL_EL0",
        (3, 14, 5, 2, 1) => "CNTP_CTL_EL02",
        (3, 14, 3, 2, 2) => "CNTP_CVAL_EL0",
        (3, 14, 5, 2, 2) => "CNTP_CVAL_EL02",
        (3, 14, 3, 2, 0) => "CNTP_TVAL_EL0",
        (3, 14, 5, 2, 0) => "CNTP_TVAL_EL02",
        (3, 14, 3, 0, 1) => "CNTPCT_EL0",
        (3, 14, 7, 2, 1) => "CNTPS_CTL_EL1",
        (3, 14, 7, 2, 2) => "CNTPS_CVAL_EL1",
        (3, 14, 7, 2, 0) => "CNTPS_TVAL_EL1",
        (3, 14, 3, 3, 1) => "CNTV_CTL_EL0",
        (3, 14, 5, 3, 1) => "CNTV_CTL_EL02",
        (3, 14, 3, 3, 2) => "CNTV_CVAL_EL0",
        (3, 14, 5, 3, 2) => "CNTV_CVAL_EL02",
        (3, 14, 3, 3, 0) => "CNTV_TVAL_EL0",
        (3, 14, 5, 3, 0) => "CNTV_TVAL_EL02",
        (3, 14, 3, 0, 2) => "CNTVCT_EL0",
        (3, 14, 4, 0, 3) => "CNTVOFF_EL2",
        (3, 13, 0, 0, 1) => "CONTEXTIDR_EL1",
        (3, 13, 5, 0, 1) => "CONTEXTIDR_EL12",
        (3, 13, 4, 0, 1) => "CONTEXTIDR_EL2",
        (3, 1, 5, 0, 2) => "CPACR_EL12",
        (3, 3, 4, 0, 0) => "DACR32_EL2",
        (3, 5, 5, 2, 0) => "ESR_EL12",
        (3, 6, 0, 0, 0) => "FAR_EL1",
        (3, 6, 5, 0, 0) => "FAR_EL12",
        (3, 6, 4, 0, 0) => "FAR_EL2",
        (3, 6, 6, 0, 0) => "FAR_EL3",
        (3, 5, 4, 3, 0) => "FPEXC32_EL2",
        (3, 6, 4, 0, 4) => "HPFAR_EL2",
        (3, 1, 4, 1, 3) => "HSTR_EL2",
        (3, 0, 0, 5, 4) => "ID_AA64AFR0_EL1",
        (3, 0, 0, 5, 5) => "ID_AA64AFR1_EL1",
        (3, 0, 0, 5, 1) => "ID_AA64DFR1_EL1",
        (3, 0, 0, 4, 1) => "ID_AA64PFR1_EL1",
        (3, 12, 0, 1, 0) => "ISR_EL1",
        (3, 10, 0, 4, 1) => "LOREA_EL1",
        (3, 10, 0, 4, 0) => "LORSA_EL1",
        (3, 10, 0, 2, 0) => "MAIR_EL1",
        (3, 10, 5, 2, 0) => "MAIR_EL12",
        (3, 10, 4, 2, 0) => "MAIR_EL2",
        (3, 10, 6, 2, 0) => "MAIR_EL3",
        (3, 1, 4, 1, 1) => "MDCR_EL2",
        (3, 0, 0, 3, 0) => "MVFR0_EL1",
        (3, 0, 0, 3, 1) => "MVFR1_EL1",
        (3, 0, 0, 3, 2) => "MVFR2_EL1",
        (3, 12, 6, 0, 2) => "RMR_EL3",
        (3, 1, 6, 1, 0) => "SCR_EL3",
        (3, 1, 5, 0, 0) => "SCTLR_EL12",
        (3, 1, 4, 0, 0) => "SCTLR_EL2",
        (3, 1, 6, 1, 1) => "SDER32_EL3",
        (3, 2, 5, 0, 2) => "TCR_EL12",
        (3, 13, 3, 0, 2) => "TPIDR_EL0",
        (3, 13, 0, 0, 4) => "TPIDR_EL1",
        (3, 13, 4, 0, 2) => "TPIDR_EL2",
        (3, 13, 6, 0, 2) => "TPIDR_EL3",
        (3, 13, 3, 0, 3) => "TPIDRRO_EL0",
        (3, 2, 5, 0, 0) => "TTBR0_EL12",
        (3, 2, 5, 0, 1) => "TTBR1_EL12",
        (3, 12, 0, 0, 0) => "VBAR_EL1",
        (3, 12, 5, 0, 0) => "VBAR_EL12",
        (3, 12, 4, 0, 0) => "VBAR_EL2",
        (3, 12, 6, 0, 0) => "VBAR_EL3",
        (3, 0, 4, 0, 5) => "VMPIDR_EL2",
        (3, 0, 4, 0, 0) => "VPIDR_EL2",
        _ => "unknown",
    }
}
