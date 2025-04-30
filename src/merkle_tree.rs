use std::hash::{DefaultHasher, Hash, Hasher};

pub struct MerkleTree {
    tree: Vec<Vec<u64>>,
    data_amount: usize, // Data set by the user
}

impl MerkleTree {
    /// Creates a new tree from an array of hashable data
    pub fn new<H: Hash>(data: &[H]) -> MerkleTree {
        // Tree representation + leaf vector
        let mut leaves: Vec<u64> = Vec::new();
        // Hash the user data and push it into the vector
        for d in data {
            leaves.push(hash_value(d));
        }
        // Check if the amount of leaves is a power of two, if not complete it with 0
        add_padding(&mut leaves, data.len().next_power_of_two());
        let hashed_tree = generate_tree_from_hashes(&mut leaves);
        MerkleTree {
            tree: hashed_tree,
            data_amount: data.len(),
        }
    }
    /// Dinamically add a new element into the tree
    /// The element is added at the leaves element
    /// If there is padding, the value takes the position of the first padding element
    /// If there isn't padding, generate a subtree with the same amount of leaves as the original tree and merge it into the original one
    pub fn push<H: Hash>(&mut self, data: &H) {
        // If the last level of the tree is bigger or equal than the amount of inputed data leaves then the level has padding, the value can be overwritten
        if self.tree[0].len() > self.data_amount {
            let mut index = self.data_amount;
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
                        &self.tree[level - 1][index - 1],
                        &self.tree[level - 1][index],
                    );
                }
                if self.tree[level].len() == 1 {
                    break;
                }
                level += 1;
                index /= 2;
            }
        } else {
            // If there wasn't any free space then the amount of leaves will duplicate
            let mut leaves = vec![hash_value(data)];
            // Add padding so that both subtrees have the same amount of leaves
            add_padding(&mut leaves, self.tree[0].len());
            let right_subtree = generate_tree_from_hashes(&mut leaves);
            merge_trees(&mut self.tree, right_subtree);
        }
        self.data_amount += 1;
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
            if level_index == self.tree.len() - 1 {
                break;
            }
        }
        proof
    }
    /// Verifies if `value` is at `index` with the given `proof`
    pub fn verify<H: Hash>(&self, value: H, index: usize, proof: &mut Vec<u64>) -> bool {
        let mut mutable_index = index;
        let mut hashed_value = hash_value(&value);
        for i in proof {
            // Its pointing to the left child
            if mutable_index % 2 == 0 {
                hashed_value = hash_two_values(&hashed_value, i);
            }
            // Its pointing to the right child
            else {
                hashed_value = hash_two_values(i, &hashed_value);
            }
            mutable_index /= 2;
        }
        hashed_value == self.tree[self.tree.len() - 1][0]
    }

    pub fn get_tree(&self) -> Vec<Vec<u64>> {
        self.tree.clone()
    }
}

pub fn hash_value<H: Hash>(data: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*data).hash(&mut hasher);
    hasher.finish()
}
/// Receives 2 values and returns their combined hash, order matters
pub fn hash_two_values<H: Hash>(left_child: &H, right_child: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*left_child).hash(&mut hasher);
    (*right_child).hash(&mut hasher);
    hasher.finish()
}
/// Add hashed 0s up to a desire size
pub fn add_padding(tree_level: &mut Vec<u64>, up_to: usize) {
    if !tree_level.is_empty() {
        loop {
            if up_to == tree_level.len() {
                break;
            }
            tree_level.push(hash_value(&0));
        }
    }
}
/// Generates a full tree from a list of hashed leaves.
/// It returns a list of vectors, each representing a level of the tree
pub fn generate_tree_from_hashes(leaves: &mut [u64]) -> Vec<Vec<u64>> {
    let mut tree = Vec::new();
    tree.push(leaves.to_vec());
    // Calculate the combined hashes and push it
    let mut level_index = 0;
    if leaves.len() > 1 {
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
    tree
}
/// Merges the second tree into the first one and calculates the new root
// Given 2 full binary trees merge the second one into the first one, appending each level together and calculating the new root
pub fn merge_trees(left_subtree: &mut Vec<Vec<u64>>, mut right_subtree: Vec<Vec<u64>>) {
    // Panic if trees haven't the same amount of floors
    if left_subtree.len() != right_subtree.len() {
        panic!("Trees must have the same amount of floors");
    }
    let len = left_subtree.len();
    let mut index = 0;
    loop {
        left_subtree[index].append(&mut right_subtree[index]);
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
    use crate::{MerkleTree, generate_tree_from_hashes, hash_value, merge_trees};

    #[test]
    fn creation_test() {
        let not_power_of_two_tree = MerkleTree::new(&[0; 5]);
        let closest_power_of_two_tree = MerkleTree::new(&[0; 8]);
        assert_eq!(
            not_power_of_two_tree.get_tree(),
            closest_power_of_two_tree.get_tree()
        );
    }
    #[test]
    fn merge_test() {
        let hashed_0 = hash_value(&0);
        let hashed_1 = hash_value(&1);
        let mut left_subtree = generate_tree_from_hashes(&mut vec![hashed_0; 4]);
        let right_subtree = generate_tree_from_hashes(&mut vec![hashed_1; 4]);
        merge_trees(&mut left_subtree, right_subtree);
        let res = generate_tree_from_hashes(&mut vec![
            hashed_0, hashed_0, hashed_0, hashed_0, hashed_1, hashed_1, hashed_1, hashed_1,
        ]);
        assert_eq!(res, left_subtree);
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
    #[test]
    fn verify_pass() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut proof = tree.generate_proof(2);
        assert_eq!(tree.verify(3, 2, &mut proof), true);
    }

    #[test]
    fn verify_fail() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut proof = tree.generate_proof(2);
        assert_eq!(tree.verify(5, 2, &mut proof), false);
        assert_eq!(tree.verify(3, 5, &mut proof), false);
        assert_eq!(tree.verify(5, 5, &mut proof), false);
        assert_eq!(
            tree.verify(3, 2, &mut vec![tree.tree[0][1], tree.tree[1][3]]),
            false
        );
    }
}
