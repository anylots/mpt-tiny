use mpt_tiny::mpt::Tree;
use mpt_tiny::node::MptNode;

#[test]
fn test_load() {
    let expected = hex::decode("e4850001020304ddc882350684636f696e8080808080808080808080808080808476657262").unwrap();

    let mpt_node = MptNode::from(&expected);
    Tree::new(mpt_node);
}
