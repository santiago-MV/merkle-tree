use merkle_tree::MerkleTree;

#[test]
fn usage_test() {
    let mut tree = MerkleTree::new(&[0; 7]);
    print!("{}", 7_usize.next_power_of_two());
    tree.push(&5); // Index 7
    assert_eq!(tree.get_tree()[0].len(), 8);
    tree.push(&10); // Index 8
    assert_eq!(tree.get_tree()[0].len(), 16);
    let mut proof = tree.generate_proof(8);
    assert!(tree.verify(10, 8, &mut proof));
}
