# merkle-tree
An implementation of a Merkle tree using Rust

## Dependencies
rust = "1.86.0"

## Usage

`MerkleTree` struct that implements:
* `new(data: &[H]) -> MerkleTree`: Creates a merkle tree form a slice of data.
* `push(value: &H)`: Dinamically adds a leaf to the tree (only recalculates necessary nodes)
* `generate_proof(index: usize) -> Vec<u64>`: Generates a proof for the index leaf. The proof is a vector that contains the hashes needed to verify a node
* `verify(value: &H,index: usize,proof: Vec<u64>) -> bool`: Verifies if the value is the leaf at index with the given proof
* `get_tree() -> & Vec<Vec<u64>>`: Returns a reference to the current state of the tree
* `get_root() -> u64`: Returns the root of the tree

### Example
In this example we'll create a new merkle tree form an array of values, then use the push function to add new data to the tree, generate a proof for the new leaf and verify that it was added.
``` rust
let data = [10,20,7,25,17,89,88,99];            // Define data
let mut tree = MerkleTree::new(&data);          // Create the new Merkle tree   
let _ = match tree.push(&5){                    // Push a new value into the tree (Position 8)
    Ok(_) => (),            
    Err(message) => panic!("{}", message),      // Doesn't need to panic
};                                 
let mut proof = match tree.generate_proof(8){   // Generate a proof for the value at position 8
        Ok(generated_proof) => generated_proof, 
        Err(message) => panic!("{}", message),  // Doesn't need to panic
    };         
let verification = tree.verify(&5,8,proof);     // Check that the value pushed is the one at position 
```
This example can is tested in the example_test
## Instalation

Clone the repo and run `make build`

### Other commands

* `make test`: Run all the tests
* `make clippy`: Check for clippy errors
* `make fmt`: Check for format errors
* `make docs`: Generate and open cargo docs
