# 1. Setup
## Verification tools
```
cargo install -f cargo-bolero
cargo install --locked kani-verifier
cargo kani setup
```

## Install Rust 
```
apt update
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

# 2. Run tool with helper script
```
python3 evaluation.py benchmark_language benchmark_name verification_tool
# e.g., python3 evaluation.py c BIRTHDAY_PARADOX bounded_kani
# Choose a benchmark_name from c_results.csv, where c is the source language
```

# 3. Run yourself without helper script (example project )
## Bolero
```
cd benchmark/c_transcoder/BIRTHDAY_PARADOX/out-rwasm-bolero
cargo bolero reduce bolero_wasm_eq
```
## Bounded kani
```
cd benchmark/c_transcoder/BIRTHDAY_PARADOX/out-rwasm-mutated
cargo kani --no-unwinding-checks --default-unwind 10
```
## Bolero
```
cd benchmark/c_transcoder/BIRTHDAY_PARADOX/out-rwasm-mutated
cargo kani
```

# 4. Remake all evaluation data from scratch
Note this takes a significant amount of time. For C the entire run takes about 15 hours.
```
python3 torust benchmark_language
# e.g., python3 torust cpp
```
