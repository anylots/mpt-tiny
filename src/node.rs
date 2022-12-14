use crate::util;
use serde::{Deserialize, Serialize};
use serlp::{
    de::RlpProxy,
    rlp::{from_bytes, to_bytes, RlpNodeValue},
    types::byte_array,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum MptNode {
    Leaf(LeafNode),
    Extension(ExtensionNode),
    Branch(BranchNode),
}

impl MptNode {
    //prepare data for storage
    pub fn parse(&self) -> (util::KecHash, Vec<u8>) {
        let value = serlp::rlp::to_bytes(self).unwrap();
        return (util::kecak256(&value), value);
    }

    //from rlp data
    pub fn from(data: &[u8]) -> Self {
        return serlp::rlp::from_bytes(data).unwrap();
    }
}

impl From<LeafNode> for MptNode {
    fn from(node: LeafNode) -> Self {
        Self::Leaf(node)
    }
}

impl From<ExtensionNode> for MptNode {
    fn from(node: ExtensionNode) -> Self {
        Self::Extension(node)
    }
}

impl From<BranchNode> for MptNode {
    fn from(node: BranchNode) -> Self {
        Self::Branch(node)
    }
}

pub struct EmptyNode {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LeafNode {
    #[serde(with = "byte_array")]
    kecak_hash: util::KecHash,
    #[serde(with = "serde_bytes")]
    value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BranchNode {
    pub branchs: [Box<MptNode>; 16],
    #[serde(with = "serde_bytes")]
    value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ExtensionNode {
    #[serde(with = "byte_array")]
    pub kecak_hash: util::KecHash,
    #[serde(with = "byte_array")]
    pub value: util::KecHash,
}
