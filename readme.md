# 1. Setup

## Install Rust and Clang
```
apt update
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
apt install clang
export PATH=/usr/local/clang/bin:$PATH
```

## Checking Tools
```
cargo install cargo-bolero --git https://github.com/YoshikiTakashima/bolero.git --branch kani-unwind-0
cargo install --locked kani-verifier@0.55.0
cargo kani setup
```

## Python requirements
```
pip install -r requirements.txt
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
## Full Kani
```
cd benchmark/c_transcoder/BIRTHDAY_PARADOX/out-rwasm-mutated
cargo kani
```

# 4. Remake all evaluation data from scratch
Note this takes a significant amount of time. For C the entire run takes about 15 hours.
```
python3 torust benchmark_language
# e.g., 
python3 torust.py --aws-profile default --language c --llm-attempts 5 --benchmark-dir benchmark/c_transcoder/BIRTHDAY_PARADOX
```
