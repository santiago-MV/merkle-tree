mod merkle_tree;
fn main() {
    let test_tree = merkle_tree::new(&[0; 5]);
    print!("{:?}", test_tree.get_tree());
}
