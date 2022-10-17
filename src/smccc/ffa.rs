const FFA_ERROR: u64 = 0x60;
const FFA_SUCCESS: u64 = 0x61;
const FFA_INTERRUPT: u64 = 0x62;
const FFA_VERSION: u64 = 0x63;
const FFA_FEATURES: u64 = 0x64;
const FFA_RX_RELEASE: u64 = 0x65;
const FFA_RXTX_MAP: u64 = 0x66;
const FFA_RXTX_UNMAP: u64 = 0x67;
const FFA_PARTITION_INFO_GET: u64 = 0x68;
const FFA_ID_GET: u64 = 0x69;
const FFA_MSG_POLL: u64 = 0x6A;
const FFA_MSG_WAIT: u64 = 0x6B;
const FFA_YIELD: u64 = 0x6C;
const FFA_RUN: u64 = 0x6D;
const FFA_MSG_SEND: u64 = 0x6E;
const FFA_MSG_SEND_DIRECT_REQ: u64 = 0x6F;
const FFA_MSG_SEND_DIRECT_RESP: u64 = 0x70;
const FFA_MEM_DONATE: u64 = 0x71;
const FFA_MEM_LEND: u64 = 0x72;
const FFA_MEM_SHARE: u64 = 0x73;
const FFA_MEM_RETRIEVE_REQ: u64 = 0x74;
const FFA_MEM_RETRIEVE_RESP: u64 = 0x75;
const FFA_MEM_RELINQUISH: u64 = 0x76;
const FFA_MEM_RECLAIM: u64 = 0x77;
const FFA_MEM_OP_PAUSE: u64 = 0x78;
const FFA_MEM_OP_RESUME: u64 = 0x79;
const FFA_MEM_FRAG_RX: u64 = 0x7A;
const FFA_MEM_FRAG_TX: u64 = 0x7B;
const FFA_NORMAL_WORLD_RESUME: u64 = 0x7C;

pub fn ffa_32_function_id(function: u64) -> Option<&'static str> {
    match function {
        FFA_ERROR => Some("FFA_ERROR_32"),
        FFA_SUCCESS => Some("FFA_SUCCESS_32"),
        FFA_INTERRUPT => Some("FFA_INTERRUPT_32"),
        FFA_VERSION => Some("FFA_VERSION_32"),
        FFA_FEATURES => Some("FFA_FEATURES_32"),
        FFA_RX_RELEASE => Some("FFA_RX_RELEASE_32"),
        FFA_RXTX_MAP => Some("FFA_RXTX_MAP_32"),
        FFA_RXTX_UNMAP => Some("FFA_RXTX_UNMAP_32"),
        FFA_PARTITION_INFO_GET => Some("FFA_PARTITION_INFO_GET_32"),
        FFA_ID_GET => Some("FFA_ID_GET_32"),
        FFA_MSG_POLL => Some("FFA_MSG_POLL_32"),
        FFA_MSG_WAIT => Some("FFA_MSG_WAIT_32"),
        FFA_YIELD => Some("FFA_YIELD_32"),
        FFA_RUN => Some("FFA_RUN_32"),
        FFA_MSG_SEND => Some("FFA_MSG_SEND_32"),
        FFA_MSG_SEND_DIRECT_REQ => Some("FFA_MSG_SEND_DIRECT_REQ_32"),
        FFA_MSG_SEND_DIRECT_RESP => Some("FFA_MSG_SEND_DIRECT_RESP_32"),
        FFA_MEM_DONATE => Some("FFA_MEM_DONATE_32"),
        FFA_MEM_LEND => Some("FFA_MEM_LEND_32"),
        FFA_MEM_SHARE => Some("FFA_MEM_SHARE_32"),
        FFA_MEM_RETRIEVE_REQ => Some("FFA_MEM_RETRIEVE_REQ_32"),
        FFA_MEM_RETRIEVE_RESP => Some("FFA_MEM_RETRIEVE_RESP_32"),
        FFA_MEM_RELINQUISH => Some("FFA_MEM_RELINQUISH_32"),
        FFA_MEM_RECLAIM => Some("FFA_MEM_RECLAIM_32"),
        FFA_MEM_OP_PAUSE => Some("FFA_MEM_OP_PAUSE"),
        FFA_MEM_OP_RESUME => Some("FFA_MEM_OP_RESUME"),
        FFA_MEM_FRAG_RX => Some("FFA_MEM_FRAG_RX_32"),
        FFA_MEM_FRAG_TX => Some("FFA_MEM_FRAG_TX_32"),
        FFA_NORMAL_WORLD_RESUME => Some("FFA_NORMAL_WORLD_RESUME"),
        _ => None,
    }
}

pub fn ffa_64_function_id(function: u64) -> Option<&'static str> {
    match function {
        FFA_RXTX_MAP => Some("FFA_RXTX_MAP_64"),
        FFA_MSG_SEND_DIRECT_REQ => Some("FFA_MSG_SEND_DIRECT_REQ_64"),
        FFA_MSG_SEND_DIRECT_RESP => Some("FFA_MSG_SEND_DIRECT_RESP_64"),
        FFA_MEM_DONATE => Some("FFA_MEM_DONATE_64"),
        FFA_MEM_LEND => Some("FFA_MEM_LEND_64"),
        FFA_MEM_SHARE => Some("FFA_MEM_SHARE_64"),
        FFA_MEM_RETRIEVE_REQ => Some("FFA_MEM_RETRIEVE_REQ_64"),
        _ => None,
    }
}
