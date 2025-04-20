# vanity-address-generate
The Vanity Address Generator is a high-performance Rust application for generating Ethereum vanity addresses with a specified number of leading zeros. It utilizes the Create2 contract to compute contract addresses deterministically using the Create2 opcode. By generating random salts and checking the resulting addresses in parallel, the tool efficiently finds addresses with desired prefixes `(e.g., 0x00000000...)`. 



## Installation
Clone the Repository:
```
git clone https://github.com/0x0077/vanity-address-generate.git
cd vanity-address-generator
```

## Usage

- Update the following constants in `src/main.rs`:
```rust
const FACTORY_ADDRESS: &str = "0x000000f2529cafe47f13bc4d674e343a97a870c1"; 
const INIT_CODE_HASH: &str = "0x<your_init_code_hash>"; // Keccak256 hash of the contract's init code
```

- Set the target number of leading zeros in the `main` function:
```rust
let target_zeros = 16; // Number of leading zeros
```

- Run
```rust
cargo run
```


## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

