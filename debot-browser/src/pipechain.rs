use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default;

fn default_init_method() -> String {
    format!("start")
}
fn default_mandatory() -> bool {
    false
}

#[derive(Deserialize, Clone, PartialEq)]
pub enum ApproveKind {
    ApproveOnChainCall,
    ApproveNetwork,
    ApproveMessageLimit,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DebotManifest {
    pub version: u8,
    pub debot_address: String,
    #[serde(default = "default_init_method")]
    pub init_method: String,
    pub init_args: Option<Value>,
    pub init_msg: Option<String>,
    pub abi: Option<Value>,
    pub auto_approve: Option<Vec<ApproveKind>>,
    pub quiet: bool,
    pub chain: Vec<ChainLink>,
}

impl DebotManifest {
    pub fn new() -> Self {
        let mut pipechain = DebotManifest::default();
        pipechain.init_method = default_init_method();
        pipechain.quiet = false;
        pipechain
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ChainLink {
    Input {
        interface: String,
        method: String,
        params: Option<Value>,
        #[serde(default = "default_mandatory")]
        mandatory: bool,
    },
    OnchainCall {
        approve: bool,
        iflq: Option<String>,
        ifeq: Option<String>,
    },
    SigningBox {
        handle: u32,
    },
}