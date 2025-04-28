use std::hash::{DefaultHasher, Hash, Hasher};
pub struct MerkleTree {
    tree: Vec<u64>,
}
impl MerkleTree {
    fn generate_tree<H: Hash>(&mut self, data: &[H]) {
        // Vector that will hold the hashed values
        let mut hashed_tree: Vec<u64> = Vec::new();
        // Hash the user data and push it into the vector
        for d in data {
            let mut hasher = DefaultHasher::new();
            (*d).hash(&mut hasher);
            hashed_tree.push(hasher.finish());
        }
        // Check if the amount of nodes is a power of two, if not complete it with 0
        if !is_power_of_2(hashed_tree.len() as u128) {
            let power_of_2 = closest_power_of_2(hashed_tree.len() as u128);
            loop {
                if power_of_2 == hashed_tree.len() as u128 {
                    break;
                }
                let mut hasher = DefaultHasher::new();
                0.hash(&mut hasher);
                hashed_tree.push(hasher.finish());
            }
        }
        // Reverse hashed_tree so that the combined hashes can be pushed
        hashed_tree.reverse();
        // Calculate the amount of nodes the binary tree will have
        // number_of_nodes = 2 * amount_of_leafs - 1
        let number_of_nodes = 2 * hashed_tree.len() - 1;
        // Calculate the combined hashes and push it
        let mut index = 0;
        loop {
            let mut hasher = DefaultHasher::new();
            (hashed_tree[index] as u128 + hashed_tree[index + 1] as u128).hash(&mut hasher);
            hashed_tree.push(hasher.finish());
            index += 2;
            if index >= number_of_nodes - 1 {
                break;
            }
        }
        // Reverse the vector so that the root is the head
        hashed_tree.reverse();
        // Set the value into the tree
        self.tree = hashed_tree.clone();
    }

    pub fn get_tree(&self) -> Vec<u64> {
        self.tree.clone()
    }
}
// Creates a new MekleTree and generates the tree from the argument array
pub fn new<H: Hash>(data: &[H]) -> MerkleTree {
    let mut initialized_tree = MerkleTree { tree: Vec::new() };
    initialized_tree.generate_tree(data);
    initialized_tree
}

fn is_power_of_2(number: u128) -> bool {
    let mut value = number;
    loop {
        // If the value is 2 then return true
        if value == 2 {
            return true;
        }
        // if the value is odd return falshe
        if value % 2 != 0 {
            return false;
        }
        // Neither of the above then divide the value by 2 and continue
        value /= 2;
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
}
