use std::hash::{DefaultHasher, Hash, Hasher};
pub struct MerkleTree {
    tree: Vec<Vec<u64>>,
    leaf_amount: usize,
}
impl MerkleTree {
    /// Creates a new tree from an array of hashable data
    pub fn new<H: Hash>(data: &[H]) -> MerkleTree {
        // Tree representation + leaf vector
        let mut hashed_tree: Vec<Vec<u64>> = Vec::new();
        let mut leafs: Vec<u64> = Vec::new();
        // Hash the user data and push it into the vector
        for d in data {
            leafs.push(hash_value(d));
        }
        // Check if the amount of leafs is a power of two, if not complete it with 0
        add_padding(&mut leafs, closest_power_of_2(data.len()));
        hashed_tree.push(leafs);
        generate_from_hashes(&mut hashed_tree);
        MerkleTree {
            tree: hashed_tree,
            leaf_amount: data.len(),
        }
    }
    /// Adds a new leaf into the tree, regenerating the full tree
    pub fn push<H: Hash>(&mut self, data: &H) {
        // If the last level of the tree is bigger or equal than the amount of inputed data leafs then the level has padding, the value can be overwritten
        if self.tree[0].len() > self.leaf_amount {
            let mut index = self.leaf_amount + 1;
            let mut level = 1;
            self.tree[0][index] = hash_value(data);
            // Recalculate necessary nodes
            loop {
                // Index is pointing to the left child
                if index % 2 == 0 {
                    self.tree[level][index / 2] = hash_two_values(
                        &self.tree[level - 1][index],
                        &self.tree[level - 1][index + 1],
                    );
                }
                // Index pointing to the right child
                else {
                    self.tree[level][index / 2] = hash_two_values(
                        &self.tree[level - 1][index-1],
                        &self.tree[level - 1][index],
                    );
                }
                level += 1;
                index /= 2;
                if self.tree[level].len() == 1 {
                    break;
                }
            }
        } else {
            // If there wasn't padding, there wasn't free space then push the value and add padding
            
            // The amount of elements in the last level has duplicated, every node in the current tree is now part of the left subtree
            // Calculate the right subtree
            let mut right_subtree = Vec::new();
            let mut new_leafs = vec![hash_value(data)];
            // The array will duplicate its value
            add_padding(&mut new_leafs,self.tree[0].len());
            print!("{:?}",new_leafs);
            right_subtree.push(new_leafs);
            generate_from_hashes(&mut right_subtree);
            merge_trees(&mut self.tree, right_subtree);
        }
    }
    /// Given the index of the data that wants to be validated, generate the array of hashes needed to validate that position
    pub fn generate_proof(&self, index: usize) -> Vec<u64> {
        if index > self.tree[0].len() {
            panic!("Index out of bounds");
        }
        let mut proof = Vec::new();
        let mut mutable_index = index;
        let mut level_index = 0;
        for _ in self.get_tree() {
            // If actual index is even, then is pointing to the left child, push the right one
            if mutable_index % 2 == 0 {
                proof.push(self.tree[level_index][mutable_index + 1]);
            }
            // If its odd, its pointing to the right child, push the left child one
            else {
                proof.push(self.tree[level_index][mutable_index - 1]);
            }
            // Update the index for the next level
            mutable_index /= 2;
            level_index += 1;
            if level_index == self.tree.len()-1{
                break;
            }
        }
        proof
    }

    pub fn verify<H:Hash>(&self,value:H,index:usize,proof: &mut Vec<u64>) -> bool{
        let mut mutable_index = index;
        let mut hashed_value = hash_value(&value);
        for i in proof {
            // Its pointing to the left child
            if mutable_index % 2 == 0 {
                hashed_value = hash_two_values(&hashed_value, i);
            }
            // Its pointing to the right child
            else {
               hashed_value = hash_two_values(i,&hashed_value);
            }
            mutable_index /= 2;
        }
        hashed_value == self.tree[self.tree.len()-1][0]
    }

    pub fn get_tree(&self) -> Vec<Vec<u64>> {
        self.tree.clone()
    }
}

fn closest_power_of_2(number: usize) -> usize {
    number.next_power_of_two()
}

fn hash_value<H: Hash>(data: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*data).hash(&mut hasher);
    hasher.finish()
}

fn hash_two_values<H: Hash>(left_child: &H, right_child: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*left_child).hash(&mut hasher);
    (*right_child).hash(&mut hasher);
    hasher.finish()
}
// If the leaf amount isn't a power of 2 then add the minimum amount of 0 possible to make it one
fn add_padding(tree_level: &mut Vec<u64>,up_to: usize) {
    if tree_level.len() == 1 || !tree_level.len().is_power_of_two(){
        loop {
            if up_to == tree_level.len(){
                break;
            }
            tree_level.push(hash_value(&0));
        }
    }
}
/// Generates the full tree from an array of hashed leafs
fn generate_from_hashes(tree: &mut Vec<Vec<u64>>) {
    // Calculate the combined hashes and push it
    let mut level_index = 0;
    loop {
        let mut next_level = Vec::new();
        let mut index = 0;
        loop {
            let left_data = tree[level_index][index];
            let right_data = tree[level_index][index + 1];
            next_level.push(hash_two_values(&left_data, &right_data));
            index += 2;
            if tree[level_index].len() <= index {
                break;
            }
        }
        level_index += 1;
        let len = next_level.len();
        tree.push(next_level);
        if len == 1 {
            break;
        }
    }
}

fn merge_trees(left_subtree: &mut Vec<Vec<u64>>, mut rigth_subtree: Vec<Vec<u64>>) {
    // Panic if trees haven't the same amount of floors
    if left_subtree.len() != rigth_subtree.len() {
        panic!("Trees must have the same amount of floors");
    }
    let len = left_subtree.len();
    let mut index = 0;
    loop {
        left_subtree[index].append(&mut rigth_subtree[index]);
        index += 1;
        if index == len {
            break;
        }
    }
    let root = vec![hash_two_values(
        &left_subtree[index - 1][0],
        &left_subtree[index - 1][1],
    )];
    left_subtree.push(root);
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::MerkleTree;

    #[test]
    fn creation_test() {
        let not_power_of_two_tree = MerkleTree::new(&[0; 5]);
        let closes_power_of_two_tree = MerkleTree::new(&[0; 8]);
        assert_eq!(
            not_power_of_two_tree.get_tree(),
            closes_power_of_two_tree.get_tree()
        );
    }
    #[test]
    fn push_test() {
        let mut first_tree = MerkleTree::new(&[0, 0, 0, 0]);
        let second_tree = MerkleTree::new(&[0, 0, 0, 0, 1]);
        first_tree.push(&1);
        assert_eq!(first_tree.get_tree(), second_tree.get_tree());
    }
    
    #[test]
    fn generate_proof_left() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(7);
        let mut test_proof = Vec::new();
        test_proof.push(tree.tree[0][6]);
        test_proof.push(tree.tree[1][2]);
        test_proof.push(tree.tree[2][0]);
        assert_eq!(proof, test_proof)
    }
    #[test]
    fn generate_proof_right() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(2);
        let mut test_proof = Vec::new();
        test_proof.push(tree.tree[0][3]);
        test_proof.push(tree.tree[1][0]);
        test_proof.push(tree.tree[2][1]);
        assert_eq!(proof, test_proof)
    }
}
