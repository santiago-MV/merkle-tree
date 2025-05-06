use std::hash::{DefaultHasher, Hash, Hasher};
/// The MekleTree struct is used to represent the merkleTree in the code
/// The tree itself is represented using a Vector of Vectors, in which each vector represents a level of the tree.
/// Each level of the tree has the hashes of all the nodes in that level in u64
/// The data_amount field has the number of leafs, nodes from the last level, that are data. This is useful for pushing new data into the tree.
pub struct MerkleTree {
    tree: Vec<Vec<u64>>,
    data_amount: usize,
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
    pub fn push<H: Hash>(&mut self, data: &H) -> Result<(), String> {
        // If the last level of the tree is bigger or equal than the amount of inputed data leaves then the level has padding, the value can be overwritten
        if self.tree[0].len() > self.data_amount {
            let mut index = self.data_amount;
            let mut level = 1;
            self.tree[0][index] = hash_value(data);
            // Recalculate necessary nodes
            while level >= self.tree.len() {
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
                level += 1;
                index /= 2;
            }
        } else {
            // If there wasn't any free space then the amount of leaves will duplicate
            let mut leaves = vec![hash_value(data)];
            // Add padding so that both subtrees have the same amount of leaves
            add_padding(&mut leaves, self.tree[0].len());
            let right_subtree = generate_tree_from_hashes(&mut leaves);
            match merge_trees(&mut self.tree, right_subtree) {
                Ok(_) => (),
                Err(m) => return Err(m),
            };
        }
        self.data_amount += 1;
        Ok(())
    }
    /// Given the index of the data that wants to be validated, generate the array of hashes needed to validate that position
    pub fn generate_proof(&self, mut index: usize) -> Result<Vec<u64>, String> {
        if index > self.tree[0].len() {
            return Err(String::from("Index out of bounds"));
        }
        let mut proof = Vec::new();
        let mut level_index = 0;
        let mut i = 0;
        while i < self.tree.len() {
            // If actual index is even, then is pointing to the left child, push the right one
            if index % 2 == 0 {
                proof.push(self.tree[level_index][index + 1]);
            }
            // If its odd, its pointing to the right child, push the left child one
            else {
                proof.push(self.tree[level_index][index - 1]);
            }
            // Update the index for the next level
            index /= 2;
            level_index += 1;
            if level_index == self.tree.len() - 1 {
                break;
            }
            i += 1;
        }
        Ok(proof)
    }
    /// Verifies if `value` is at `index` with the given `proof`
    pub fn verify<H: Hash>(&self, value: H, mut index: usize, proof: &mut Vec<u64>) -> bool {
        let mut hashed_value = hash_value(&value);
        for i in proof {
            // Its pointing to the left child
            if index % 2 == 0 {
                hashed_value = hash_two_values(&hashed_value, i);
            }
            // Its pointing to the right child
            else {
                hashed_value = hash_two_values(i, &hashed_value);
            }
            index /= 2;
        }
        hashed_value == self.get_root()
    }

    pub fn get_tree(&self) -> &Vec<Vec<u64>> {
        &self.tree
    }

    pub fn get_root(&self) -> u64 {
        self.tree[self.tree.len() - 1][0]
    }
}

fn hash_value<H: Hash>(data: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*data).hash(&mut hasher);
    hasher.finish()
}
/// Receives 2 values and returns their combined hash, order matters
fn hash_two_values<H: Hash>(left_child: &H, right_child: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    (*left_child).hash(&mut hasher);
    (*right_child).hash(&mut hasher);
    hasher.finish()
}
/// Add hashed 0s up to a desire size
fn add_padding(tree_level: &mut Vec<u64>, up_to: usize) {
    if !tree_level.is_empty() {
        while tree_level.len() < up_to {
            tree_level.push(hash_value(&0));
        }
    }
}
/// Generates a full tree from a list of hashed leaves.
/// It returns a list of vectors, each representing a level of the tree
fn generate_tree_from_hashes(leaves: &mut [u64]) -> Vec<Vec<u64>> {
    let mut tree = Vec::new();
    tree.push(leaves.to_vec());
    // Calculate the combined hashes and push it
    let mut level_index = 0;
    if leaves.len() > 1 {
        let mut len = 0;
        while len != 1 {
            let mut next_level = Vec::new();
            let mut index = 0;
            while tree[level_index].len() > index {
                let left_data = tree[level_index][index];
                let right_data = tree[level_index][index + 1];
                next_level.push(hash_two_values(&left_data, &right_data));
                index += 2;
            }
            level_index += 1;
            len = next_level.len();
            tree.push(next_level);
        }
    }
    tree
}
/// Merges the second tree into the first one and calculates the new root
// Given 2 full binary trees merge the second one into the first one, appending each level together and calculating the new root
fn merge_trees(
    left_subtree: &mut Vec<Vec<u64>>,
    mut right_subtree: Vec<Vec<u64>>,
) -> Result<(), String> {
    // Return an error if trees haven't the same amount of floors
    if left_subtree.len() != right_subtree.len() {
        return Err(String::from("Trees must have the same amount of floors"));
    }
    let len = left_subtree.len();
    let mut index = 0;
    while index < len {
        left_subtree[index].append(&mut right_subtree[index]);
        index += 1;
    }
    let root = vec![hash_two_values(
        &left_subtree[index - 1][0],
        &left_subtree[index - 1][1],
    )];
    left_subtree.push(root);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{MerkleTree, generate_tree_from_hashes, hash_value, merge_trees};

    #[test]
    fn creation_test() {
        let not_power_of_two_tree = MerkleTree::new(&[0; 5]);
        let closest_power_of_two_tree = MerkleTree::new(&[0; 8]);
        assert_eq!(
            not_power_of_two_tree.get_root(),
            closest_power_of_two_tree.get_root()
        );
    }
    #[test]
    fn merge_test() {
        let hashed_0 = hash_value(&0);
        let hashed_1 = hash_value(&1);
        let mut left_subtree = generate_tree_from_hashes(&mut vec![hashed_0; 4]);
        let right_subtree = generate_tree_from_hashes(&mut vec![hashed_1; 4]);
        let _ = merge_trees(&mut left_subtree, right_subtree);
        let res = generate_tree_from_hashes(&mut vec![
            hashed_0, hashed_0, hashed_0, hashed_0, hashed_1, hashed_1, hashed_1, hashed_1,
        ]);
        assert_eq!(res[res.len() - 1][0], left_subtree[res.len() - 1][0]);
    }
    #[test]
    fn push_test() {
        let mut first_tree = MerkleTree::new(&[0, 0, 0, 0]);
        let second_tree = MerkleTree::new(&[0, 0, 0, 0, 1]);
        let _ = first_tree.push(&1);
        assert_eq!(first_tree.get_root(), second_tree.get_root());
    }

    #[test]
    fn generate_proof_left() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(7).unwrap();
        let mut test_proof = Vec::new();
        test_proof.push(tree.tree[0][6]);
        test_proof.push(tree.tree[1][2]);
        test_proof.push(tree.tree[2][0]);
        assert_eq!(proof, test_proof)
    }
    #[test]
    fn generate_proof_right() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(2).unwrap();
        let mut test_proof = Vec::new();
        test_proof.push(tree.tree[0][3]);
        test_proof.push(tree.tree[1][0]);
        test_proof.push(tree.tree[2][1]);
        assert_eq!(proof, test_proof)
    }
    #[test]
    fn verify_pass() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut proof = tree.generate_proof(2).unwrap();
        assert!(tree.verify(3, 2, &mut proof));
    }

    #[test]
    fn verify_fail() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let mut proof = tree.generate_proof(2).unwrap();
        assert_eq!(tree.verify(5, 2, &mut proof), false);
        assert_eq!(tree.verify(3, 5, &mut proof), false);
        assert_eq!(tree.verify(5, 5, &mut proof), false);
        assert_eq!(
            tree.verify(3, 2, &mut vec![tree.tree[0][1], tree.tree[1][3]]),
            false
        );
    }

    #[test]
    fn index_error() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(10);
        assert_eq!(proof, Err(String::from("Index out of bounds")))
    }
}
