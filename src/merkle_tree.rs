use std::hash::{DefaultHasher, Hash, Hasher};
pub struct MerkleTree {
    tree: Vec<u64>,
    leaf_amount: usize,
}
impl MerkleTree {
    fn generate_tree<H: Hash>(&mut self, data: &[H]) {
        // Vector that will hold the hashed values
        let mut hashed_tree: Vec<u64> = Vec::new();
        // Hash the user data and push it into the vector
        for d in data {
            add_hash(d, &mut hashed_tree);
        }
        // Check if the amount of nodes is a power of two, if not complete it with 0
        add_padding(&mut hashed_tree);
        // Reverse hashed_tree so that the combined hashes can be pushed
        hashed_tree.reverse();
        self.tree = hashed_tree;
        self.generate_from_hashes();
    }

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

    pub fn get_tree(&self) -> Vec<u64> {
        self.tree.clone()
    }
}
// Creates a new instance of MerkleTree and generates its tree value from the array
pub fn new<H: Hash>(data: &[H]) -> MerkleTree {
    let mut initialized_tree = MerkleTree {
        tree: Vec::new(),
        leaf_amount: data.len(),
    };
    initialized_tree.generate_tree(data);
    initialized_tree
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
    use super::new;

    #[test]
    fn creation_test() {
        let not_power_of_two_tree = new(&[0; 5]);
        let closes_power_of_two_tree = new(&[0; 8]);
        assert_eq!(
            not_power_of_two_tree.get_tree(),
            closes_power_of_two_tree.get_tree()
        );
    }
    #[test]
    fn push_test() {
        let mut first_tree = new(&[0, 0, 0, 0]);
        let second_tree = new(&[0, 0, 0, 0, 1]);
        first_tree.push(&1);
        assert_eq!(first_tree.get_tree(), second_tree.get_tree());
    }
}
