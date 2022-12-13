

struct CommonNode{

}

struct EmptyNode{

}

struct LeafNode{
    kecak_hash: &str,
    value: Vec<u8>
}

struct BranchNode{
    branchs: Vec<CommonNode>,
    value: Vec<u8>
}

struct ExtensionNode{
    kecak_hash: &str,
    value: &str
}