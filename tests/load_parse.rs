use mpt_tiny::mpt::Tree;
use mpt_tiny::node::*;
use mpt_tiny::util;

#[test]
fn test_load() {
    let expected = hex::decode("f842a00101010101010101010101010101010101010101010101010101010101010101a00101010101010101010101010101010101010101010101010101010101010101").unwrap();

    let mut kec_hash = [1;32];
    let ext_node = ExtensionNode {kecak_hash: kec_hash, value: kec_hash};
    let mpt_node = MptNode::Extension(ext_node);
    let result = mpt_node.parse();

    println!("{:?}",result);
    println!("{:?}",hex::encode(result.clone().1));

    let mpt = MptNode::from(&result.1);
    Tree::new(mpt_node);
}
