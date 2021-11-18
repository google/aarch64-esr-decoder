use super::decode;
use crate::FieldInfo;

#[test]
fn unknown() {
    let decoded = decode(0).unwrap();
    assert_eq!(
        decoded,
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 0,
                description: Some("Unknown reason".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 0,
                description: Some("16-bit instruction trapped".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 0,
                description: None,
                subfields: vec![FieldInfo {
                    name: "RES0",
                    long_name: Some("Reserved"),
                    start: 0,
                    width: 25,
                    value: 0,
                    description: Some("ISS is RES0".to_string()),
                    subfields: vec![],
                }],
            },
        ]
    );
}

#[test]
fn data_abort() {
    assert_eq!(
        decode(0x96000050).unwrap(),
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 37,
                description: Some(
                    "Data Abort taken without a change in Exception level".to_string()
                ),
                subfields: vec![],
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 1,
                description: Some("32-bit instruction trapped".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 80,
                description: None,
                subfields: vec![
                    FieldInfo {
                        name: "ISV",
                        long_name: Some("Instruction Syndrome Valid"),
                        start: 24,
                        width: 1,
                        value: 0,
                        description: Some("No valid instruction syndrome".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "RES0",
                        long_name: Some("Reserved"),
                        start: 14,
                        width: 10,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "VNCR",
                        long_name: None,
                        start: 13,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SET",
                        long_name: Some("Synchronous Error Type"),
                        start: 11,
                        width: 2,
                        value: 0,
                        description: Some("Recoverable state (UER)".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "FnV",
                        long_name: Some("FAR not Valid"),
                        start: 10,
                        width: 1,
                        value: 0,
                        description: Some("FAR is valid".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "EA",
                        long_name: Some("External abort type"),
                        start: 9,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "CM",
                        long_name: Some("Cache Maintenance"),
                        start: 8,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "S1PTW",
                        long_name: Some("Stage-1 translation table walk"),
                        start: 7,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "WnR",
                        long_name: Some("Write not Read"),
                        start: 6,
                        width: 1,
                        value: 1,
                        description: Some("Abort caused by writing to memory".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "DFSC",
                        long_name: Some("Data Fault Status Code"),
                        start: 0,
                        width: 6,
                        value: 16,
                        description: Some(
                            "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                .to_string()
                        ),
                        subfields: vec![],
                    }
                ]
            },
        ],
    );
}

#[test]
fn data_abort_isv() {
    assert_eq!(
        decode(0x97523050).unwrap(),
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 37,
                description: Some(
                    "Data Abort taken without a change in Exception level".to_string()
                ),
                subfields: vec![],
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 1,
                description: Some("32-bit instruction trapped".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 22163536,
                description: None,
                subfields: vec![
                    FieldInfo {
                        name: "ISV",
                        long_name: Some("Instruction Syndrome Valid"),
                        start: 24,
                        width: 1,
                        value: 1,
                        description: Some("Valid instruction syndrome".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SAS",
                        long_name: Some("Syndrome Access Size"),
                        start: 22,
                        width: 2,
                        value: 1,
                        description: Some("halfword".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SSE",
                        long_name: Some("Syndrome Sign Extend"),
                        start: 21,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SRT",
                        long_name: Some("Syndrome Register Transfer"),
                        start: 16,
                        width: 5,
                        value: 18,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SF",
                        long_name: Some("Sixty-Four"),
                        start: 15,
                        width: 1,
                        value: 0,
                        description: Some("32-bit wide register".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "AR",
                        long_name: Some("Acquire/Release"),
                        start: 14,
                        width: 1,
                        value: 0,
                        description: Some("No acquire/release semantics".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "VNCR",
                        long_name: None,
                        start: 13,
                        width: 1,
                        value: 1,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SET",
                        long_name: Some("Synchronous Error Type"),
                        start: 11,
                        width: 2,
                        value: 2,
                        description: Some("Uncontainable (UC)".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "FnV",
                        long_name: Some("FAR not Valid"),
                        start: 10,
                        width: 1,
                        value: 0,
                        description: Some("FAR is valid".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "EA",
                        long_name: Some("External abort type"),
                        start: 9,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "CM",
                        long_name: Some("Cache Maintenance"),
                        start: 8,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "S1PTW",
                        long_name: Some("Stage-1 translation table walk"),
                        start: 7,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "WnR",
                        long_name: Some("Write not Read"),
                        start: 6,
                        width: 1,
                        value: 1,
                        description: Some("Abort caused by writing to memory".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "DFSC",
                        long_name: Some("Data Fault Status Code"),
                        start: 0,
                        width: 6,
                        value: 16,
                        description: Some(
                            "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                .to_string()
                        ),
                        subfields: vec![],
                    }
                ]
            }
        ],
    );
}

#[test]
fn instruction_abort() {
    assert_eq!(
        decode(0x82001e10).unwrap(),
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 32,
                description: Some("Instruction Abort from a lower Exception level".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 1,
                description: Some("32-bit instruction trapped".to_string()),
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 7696,
                description: None,
                subfields: vec![
                    FieldInfo {
                        name: "RES0",
                        long_name: Some("Reserved"),
                        start: 13,
                        width: 12,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "SET",
                        long_name: Some("Synchronous Error Type"),
                        start: 11,
                        width: 2,
                        value: 3,
                        description: Some("Restartable state (UEO)".to_string()),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "FnV",
                        long_name: Some("FAR not Valid"),
                        start: 10,
                        width: 1,
                        value: 1,
                        description: Some(
                            "FAR is not valid, it holds an unknown value".to_string()
                        ),
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "EA",
                        long_name: Some("External abort type"),
                        start: 9,
                        width: 1,
                        value: 1,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "RES0",
                        long_name: Some("Reserved"),
                        start: 8,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "S1PTW",
                        long_name: Some("Stage-1 translation table walk"),
                        start: 7,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "RES0",
                        long_name: Some("Reserved"),
                        start: 6,
                        width: 1,
                        value: 0,
                        description: None,
                        subfields: vec![],
                    },
                    FieldInfo {
                        name: "IFSC",
                        long_name: Some("Instruction Fault Status Code"),
                        start: 0,
                        width: 6,
                        value: 16,
                        description: Some(
                            "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                .to_string()
                        ),
                        subfields: vec![],
                    }
                ]
            }
        ]
    );
}

#[test]
fn sve() {
    assert_eq!(
        decode(0x1f300000).unwrap(),
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 7,
                description: Some(
                    "Trapped access to SVE, Advanced SIMD or floating point".to_string()
                ),
                subfields: vec![]
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 1,
                description: Some("32-bit instruction trapped".to_string()),
                subfields: vec![]
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 19922944,
                description: None,
                subfields: vec![
                    FieldInfo {
                        name: "CV",
                        long_name: Some("Condition code valid"),
                        start: 24,
                        width: 1,
                        value: 1,
                        description: Some("COND is valid".to_string()),
                        subfields: vec![]
                    },
                    FieldInfo {
                        name: "COND",
                        long_name: Some("Condition code of the trapped instruction"),
                        start: 20,
                        width: 4,
                        value: 3,
                        description: None,
                        subfields: vec![]
                    },
                    FieldInfo {
                        name: "RES0",
                        long_name: Some("Reserved"),
                        start: 0,
                        width: 20,
                        value: 0,
                        description: None,
                        subfields: vec![]
                    }
                ]
            },
        ]
    );
}

#[test]
fn ld64b() {
    assert_eq!(
        decode(0x2a000002).unwrap(),
        vec![
            FieldInfo {
                name: "RES0",
                long_name: Some("Reserved"),
                start: 37,
                width: 27,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "ISS2",
                long_name: None,
                start: 32,
                width: 5,
                value: 0,
                description: None,
                subfields: vec![],
            },
            FieldInfo {
                name: "EC",
                long_name: Some("Exception Class"),
                start: 26,
                width: 6,
                value: 10,
                description: Some(
                    "Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction"
                        .to_string()
                ),
                subfields: vec![]
            },
            FieldInfo {
                name: "IL",
                long_name: Some("Instruction Length"),
                start: 25,
                width: 1,
                value: 1,
                description: Some("32-bit instruction trapped".to_string()),
                subfields: vec![]
            },
            FieldInfo {
                name: "ISS",
                long_name: Some("Instruction Specific Syndrome"),
                start: 0,
                width: 25,
                value: 2,
                description: None,
                subfields: vec![FieldInfo {
                    name: "ISS",
                    long_name: None,
                    start: 0,
                    width: 25,
                    value: 2,
                    description: Some("LD64B or ST64B trapped".to_string()),
                    subfields: vec![]
                }]
            }
        ]
    );
}
