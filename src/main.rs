mod merkle_tree;
fn main() {
    let mut test_tree = merkle_tree::new(&[0; 4]);
    let test_tree_2 = merkle_tree::new(&[0; 5]);
    test_tree.push(&0);
    print!("========================================");
    print!("{:?} ", test_tree.get_tree());
    print!("========================================");
    print!("{:?} ", test_tree_2.get_tree());
    print!("========================================");
}
