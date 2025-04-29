use merkle_tree::MerkleTree;

mod merkle_tree;
fn main() {
    let mut test_tree = MerkleTree::new(&[0; 4]);
    test_tree.push(&0);
    let proof = test_tree.generate_proof(4);
    print!("========================================");
    print!("{:?} ", test_tree.get_tree());
    print!("========================================");
    print!("{:?} ", proof);
    print!("========================================");
}
