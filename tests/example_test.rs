use merkle_tree::MerkleTree;

#[test]
fn example_test() {
    let data = [10, 20, 7, 25, 17, 89, 88, 99];
    let mut tree = MerkleTree::new(&data);
    let _ = match tree.push(&5) {
        Ok(_) => (),
        Err(message) => panic!("{}", message),
    };
    let mut proof = match tree.generate_proof(8) {
        Ok(generated_proof) => generated_proof,
        Err(message) => panic!("{}", message),
    };
    let verification = tree.verify(&5, 8, &mut proof);
    assert!(verification);
}
