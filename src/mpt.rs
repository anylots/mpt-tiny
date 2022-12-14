use crate::node::*;
use crate::util;

pub struct Tree {
    root: Option<MptNode>,
    root_hash: Option<util::KecHash>,
}

impl Tree {
    pub fn new(root: MptNode) -> Self {
        Self {
            root: Some(root),
            root_hash: None,
        }
    }
}
