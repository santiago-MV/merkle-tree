use std::hash::{DefaultHasher, Hash, Hasher};
pub struct MerkleTree {
    tree: Vec<u64>,
    leaf_amount: usize,
}
impl MerkleTree {
    /// Creates a new tree from an array of hashable data
    pub fn new<H: Hash>(data: &[H]) -> MerkleTree {
        // Vector that will hold the hashed values
        let mut hashed_tree: Vec<u64> = Vec::new();
        // Hash the user data and push it into the vector
        for d in data {
            add_hash(d, &mut hashed_tree);
        }
        // Check if the amount of nodes is a power of two, if not complete it with 0
        add_padding(&mut hashed_tree);
        // Reverse hashed_tree so that the last leaf is in the head and the combined hashes can be pushed
        hashed_tree.reverse();
        let mut merkle_tree = MerkleTree {
            tree: hashed_tree,
            leaf_amount: data.len(),
        };
        merkle_tree.generate_from_hashes();
        merkle_tree
    }
    /// Generates the full tree from an array of hashed leafs
    fn generate_from_hashes(&mut self) {
        // Calculate the amount of nodes the binary tree will have
        // number_of_nodes = 2 * amount_of_leafs - 1
        let number_of_nodes = 2 * self.tree.len() - 1;
        // Calculate the combined hashes and push it
        let mut index = 0;
        loop {
            let left_data = self.tree[index];
            let right_data = self.tree[index + 1];
            add_two_hashes(&left_data, &right_data, &mut self.tree);
            index += 2;
            if index >= number_of_nodes - 1 {
                break;
            }
        }
        // Reverse the vector so that the root is the head
        self.tree.reverse();
    }
    /// Adds a new leaf into the tree, regenerating the full tree
    pub fn push<H: Hash>(&mut self, data: &H) {
        let length = self.tree.len();
        let mut leafs = self.tree[(length - self.leaf_amount)..].to_vec();
        // Add the new leaf
        add_hash(data, &mut leafs);
        // Add padding
        add_padding(&mut leafs);
        // Reverse leafs so that the last one is in the head
        leafs.reverse();
        self.tree = leafs;
        self.leaf_amount += 1;
        // Generate the rest of the tree
        self.generate_from_hashes();
    }

    pub fn generate_proof(&self, index: usize) -> Vec<u64> {
        if index > self.leaf_amount {
            panic!("Index out of bounds");
        }
        let mut proof = Vec::new();
        let offset = self.tree.len() - self.leaf_amount;
        let mut actual_index = index + offset;
        loop {
            // If actual_index points to the root then push it and break
            if actual_index == 0 {
                proof.push(self.tree[0]);
                break;
            }
            // If actual index is even, then is pointing to the left child, push the right one
            if actual_index % 2 == 0 {
                proof.push(self.tree[actual_index - 1]);
            }
            // If its odd, its pointing to the rigth child, push the left child one
            else {
                proof.push(self.tree[actual_index + 1]);
            }
            // By substracting 1 and dividing by 2 we are referencing the parent node
            // Left child = parent_node * 2 + 1
            // Right child = parent_node * 2 + 2
            // Parent_node = (child_node-1)/2
            actual_index = (actual_index - 1) / 2;
        }
        // Reverse it to pop the next hash needed for verifying
        proof.reverse();
        proof
    }

    pub fn get_tree(&self) -> Vec<u64> {
        self.tree.clone()
    }
}

fn closest_power_of_2(number: u128) -> u128 {
    let mut power = 1;
    loop {
        if number < power {
            return power;
        }
        power *= 2;
    }
}

fn add_hash<H: Hash>(data: &H, hashed_tree: &mut Vec<u64>) {
    let mut hasher = DefaultHasher::new();
    (*data).hash(&mut hasher);
    hashed_tree.push(hasher.finish());
}

fn add_two_hashes<H: Hash>(left_data: &H, right_data: &H, hashed_tree: &mut Vec<u64>) {
    let mut hasher = DefaultHasher::new();
    (*left_data).hash(&mut hasher);
    (*right_data).hash(&mut hasher);
    hashed_tree.push(hasher.finish());
}
// If the leaf amount isn't a power of 2 then add the minimum amount of 0 possible to make it one
fn add_padding(tree: &mut Vec<u64>) {
    if !tree.len().is_power_of_two() {
        let closest_power_of_2 = closest_power_of_2(tree.len() as u128);
        loop {
            if closest_power_of_2 == tree.len() as u128 {
                break;
            }
            add_hash(&0, tree);
        }
    }
}

#[cfg(test)]
mod test {
    use std::hash::DefaultHasher;

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
        test_proof.push(tree.tree[0]);
        test_proof.push(tree.tree[1]);
        test_proof.push(tree.tree[5]);
        test_proof.push(tree.tree[13]);
        assert_eq!(proof, test_proof)
    }
    #[test]
    fn generate_proof_rigth() {
        let tree = MerkleTree::new(&[1, 2, 3, 4, 5, 6, 7, 8]);
        let proof = tree.generate_proof(2);
        let mut test_proof = Vec::new();
        test_proof.push(tree.tree[0]);
        test_proof.push(tree.tree[2]);
        test_proof.push(tree.tree[3]);
        test_proof.push(tree.tree[10]);
        assert_eq!(proof, test_proof)
    }
}
