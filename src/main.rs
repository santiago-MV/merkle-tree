use merkle_tree::MerkleTree;

mod merkle_tree;
fn main() {
    let mut test_tree = MerkleTree::new(&[0; 4]);
    test_tree.push(&0);
    //test_tree.push(&556);
    //let mut proof = test_tree.generate_proof(5);

    //let verify = test_tree.verify(556, 5, &mut proof);
    print!("========================================");
    print!("{:?} ", test_tree.get_tree());
    print!("========================================");
    /*print!("{:?} ", proof);
    print!("========================================");
    print!("{:?} ", verify);
    print!("========================================");*/
}
