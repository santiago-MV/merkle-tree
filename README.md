# merkle-tree
An implementation of a Merkle tree using Rust

## Dependencies
rust = "1.86.0"

## Features

`MerkleTree` struct that implements:
* `new(data: &[H]) -> MerkleTree`: Creates a merkle tree form a slice of data.
* `push(value: &H)`: Dinamically adds a leaf to the tree (only recalculates necessary nodes)
* `generate_proof(index: usize) -> Vec<u64>`: Generates a proof for the index leaf. The proof is a vector that contains the hashes needed to verify a node
* `verify(value: &H,index: usize,proof: Vec<u64>) -> bool`: Verifies if the value is the leaf at index with the given proof
* `get_tree() -> Vec<Vec<u64>>`: Returns a copy of the current state of the tree

## Instalation

Clone the repo and run `make build`

### Other commands

* `make test`: Run all the tests
* `make clippy`: Check for clippy errors
* `make fmt`: Check for format errors
* `make docs`: Generate and open cargo docs
