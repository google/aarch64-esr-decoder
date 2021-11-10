use super::{decode, parse_number, Decoded, FieldInfo};

#[test]
fn parse_decimal() {
    assert_eq!(parse_number("12345"), Ok(12345));
}

#[test]
fn parse_hex() {
    assert_eq!(parse_number("0x123abc"), Ok(0x123abc));
}

#[test]
fn parse_invalid() {
    assert!(parse_number("123abc").is_err());
}

#[test]
fn unknown() {
    let decoded = decode(0).unwrap();
    assert_eq!(
        decoded,
        vec![
            FieldInfo {
                name: "RES0",
                start: 37,
                width: 27,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "ISS2",
                start: 32,
                width: 5,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "EC",
                start: 26,
                width: 6,
                value: 0,
                decoded: Some(Decoded {
                    description: Some("Unknown reason".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "IL",
                start: 25,
                width: 1,
                value: 0,
                decoded: Some(Decoded {
                    description: Some("16-bit instruction trapped".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "ISS",
                start: 0,
                width: 25,
                value: 0,
                decoded: Some(Decoded {
                    description: Some("ISS is RES0".to_string()),
                    fields: vec![],
                })
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
                start: 37,
                width: 27,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "ISS2",
                start: 32,
                width: 5,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "EC",
                start: 26,
                width: 6,
                value: 37,
                decoded: Some(Decoded {
                    description: Some(
                        "Data Abort taken without a change in Exception level".to_string()
                    ),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "IL",
                start: 25,
                width: 1,
                value: 1,
                decoded: Some(Decoded {
                    description: Some("32-bit instruction trapped".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "ISS",
                start: 0,
                width: 25,
                value: 80,
                decoded: Some(Decoded {
                    description: None,
                    fields: vec![
                        FieldInfo {
                            name: "ISV",
                            start: 24,
                            width: 1,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("No valid instruction syndrome".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "RES0",
                            start: 14,
                            width: 10,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "VNCR",
                            start: 13,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "SET",
                            start: 11,
                            width: 2,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("Recoverable state (UER)".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "FnV",
                            start: 10,
                            width: 1,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("FAR is valid".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "EA",
                            start: 9,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "CM",
                            start: 8,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "S1PTW",
                            start: 7,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "WnR",
                            start: 6,
                            width: 1,
                            value: 1,
                            decoded: Some(Decoded {
                                description: Some("Abort caused by writing to memory".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "DFSC",
                            start: 0,
                            width: 6,
                            value: 16,
                            decoded: Some(Decoded {
                                description: Some(
                                    "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                        .to_string()
                                ),
                                fields: vec![],
                            })
                        }
                    ]
                })
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
                start: 37,
                width: 27,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "ISS2",
                start: 32,
                width: 5,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "EC",
                start: 26,
                width: 6,
                value: 37,
                decoded: Some(Decoded {
                    description: Some(
                        "Data Abort taken without a change in Exception level".to_string()
                    ),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "IL",
                start: 25,
                width: 1,
                value: 1,
                decoded: Some(Decoded {
                    description: Some("32-bit instruction trapped".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "ISS",
                start: 0,
                width: 25,
                value: 22163536,
                decoded: Some(Decoded {
                    description: None,
                    fields: vec![
                        FieldInfo {
                            name: "ISV",
                            start: 24,
                            width: 1,
                            value: 1,
                            decoded: Some(Decoded {
                                description: Some("Valid instruction syndrome".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "SAS",
                            start: 22,
                            width: 2,
                            value: 1,
                            decoded: Some(Decoded {
                                description: Some("halfword".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "SSE",
                            start: 21,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "SRT",
                            start: 16,
                            width: 5,
                            value: 18,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "SF",
                            start: 15,
                            width: 1,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("32-bit wide register".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "AR",
                            start: 14,
                            width: 1,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("No acquire/release semantics".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "VNCR",
                            start: 13,
                            width: 1,
                            value: 1,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "SET",
                            start: 11,
                            width: 2,
                            value: 2,
                            decoded: Some(Decoded {
                                description: Some("Uncontainable (UC)".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "FnV",
                            start: 10,
                            width: 1,
                            value: 0,
                            decoded: Some(Decoded {
                                description: Some("FAR is valid".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "EA",
                            start: 9,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "CM",
                            start: 8,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "S1PTW",
                            start: 7,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "WnR",
                            start: 6,
                            width: 1,
                            value: 1,
                            decoded: Some(Decoded {
                                description: Some("Abort caused by writing to memory".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "DFSC",
                            start: 0,
                            width: 6,
                            value: 16,
                            decoded: Some(Decoded {
                                description: Some(
                                    "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                        .to_string()
                                ),
                                fields: vec![],
                            })
                        }
                    ]
                })
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
                start: 37,
                width: 27,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "ISS2",
                start: 32,
                width: 5,
                value: 0,
                decoded: None,
            },
            FieldInfo {
                name: "EC",
                start: 26,
                width: 6,
                value: 32,
                decoded: Some(Decoded {
                    description: Some("Instruction Abort from a lower Exception level".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "IL",
                start: 25,
                width: 1,
                value: 1,
                decoded: Some(Decoded {
                    description: Some("32-bit instruction trapped".to_string()),
                    fields: vec![],
                })
            },
            FieldInfo {
                name: "ISS",
                start: 0,
                width: 25,
                value: 7696,
                decoded: Some(Decoded {
                    description: None,
                    fields: vec![
                        FieldInfo {
                            name: "RES0",
                            start: 13,
                            width: 12,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "SET",
                            start: 11,
                            width: 2,
                            value: 3,
                            decoded: Some(Decoded {
                                description: Some("Restartable state (UEO)".to_string()),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "FnV",
                            start: 10,
                            width: 1,
                            value: 1,
                            decoded: Some(Decoded {
                                description: Some(
                                    "FAR is not valid, it holds an unknown value".to_string()
                                ),
                                fields: vec![],
                            })
                        },
                        FieldInfo {
                            name: "EA",
                            start: 9,
                            width: 1,
                            value: 1,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "RES0",
                            start: 8,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "S1PTW",
                            start: 7,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "RES0",
                            start: 6,
                            width: 1,
                            value: 0,
                            decoded: None,
                        },
                        FieldInfo {
                            name: "IFSC",
                            start: 0,
                            width: 6,
                            value: 16,
                            decoded: Some(Decoded {
                                description: Some(
                                    "Synchronous External abort, not on translation table \
                                         walk or hardware update of translation table."
                                        .to_string()
                                ),
                                fields: vec![],
                            })
                        }
                    ]
                })
            }
        ]
    );
}
