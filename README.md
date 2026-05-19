# cryptopals — Rust

Solutions to the [Cryptopals Crypto Challenges](https://cryptopals.com/) written in Rust.

## Why

Learning Rust and cryptography at the same time by working through each challenge from scratch — no crypto libraries, just the stdlib and a few helpers.

## Structure

```
cryptopals/
├── utils/          # shared encoding helpers (hex, base64, XOR, etc.)
├── set1/           # Set 1 — Basics
├── set2/           # Set 2 — Block Crypto
├── set3/           # Set 3 — Block and Stream Crypto
├── set4/           # Set 4 — Stream Crypto and Randomness
├── set5/           # Set 5 — Diffie-Hellman and Friends
├── set6/           # Set 6 — RSA and DSA
├── set7/           # Set 7 — Hashes
└── set8/           # Set 8 — Abstract Algebra
```

Each set is an independent Cargo library crate. Challenges live as public functions (and their `#[test]` counterparts) inside `src/lib.rs` or sub-modules.

## Running

```bash
# build everything
cargo build

# run all tests
cargo test

# run tests for a single set
cargo test -p set1

# run a specific test
cargo test -p set1 challenge_01
```

## Progress

| Set | Title | Status |
|-----|-------|--------|
| 1 | Basics | 🔄 In progress |
| 2 | Block Crypto | ⬜ Not started |
| 3 | Block and Stream Crypto | ⬜ Not started |
| 4 | Stream Crypto and Randomness | ⬜ Not started |
| 5 | Diffie-Hellman and Friends | ⬜ Not started |
| 6 | RSA and DSA | ⬜ Not started |
| 7 | Hashes | ⬜ Not started |
| 8 | Abstract Algebra | ⬜ Not started |

## Rules

- No high-level crypto crates (no `aes`, no `rsa`, etc.) — implement everything by hand.
- Each challenge gets at least one `#[test]` that verifies the expected output.
- Commits are per-challenge (e.g. `set1: challenge 1 — hex to base64`).
