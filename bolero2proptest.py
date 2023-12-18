# bolero2proptest.py --- converts a bolero harness into a proptest harness. Run
# from the root of the crate that has bolero harneses.
import subprocess
import os


def split_prefix_suffix(raw: str) -> tuple[str, str]:
    """Split the code into 2 pieces, with the second piece being the test harness."""
    split = raw.rfind("use bolero::check;")
    return (raw[:split], raw[split:])


def get_body(suffix: str) -> str:
    """Get the body of the test case."""
    start = suffix.find(")|{") + 3
    end = suffix.find("});")
    return suffix[start:end]


def get_arg_ty_pair(suffix: str) -> list[tuple[str, str]]:
    """Get the arguments and the types of them by parsing the first
    line of the bolero call."""
    ty_start = suffix.find("<(") + 2
    ty_end = suffix.find(")>")
    ty_list = suffix[ty_start:ty_end].split(", ")

    arg_start = suffix.find("|(") + 2
    arg_end = suffix.find(")|")
    arg_list = suffix[arg_start:arg_end].split(",")
    assert len(arg_list) == len(ty_list)

    return [(arg_list[i], ty_list[i]) for i in range(len(ty_list))]


def codegen_proptest(body: str, arg_ty_pairs: list[tuple[str, str]]) -> str:
    input_str = ", ".join([f"{a}: {t}" for (a, t) in arg_ty_pairs])
    return f"""
use proptest::prelude::*;
proptest!{{
  #[test]
  fn check_eq(
    {input_str}
  ) {{
    {body}
  }}
}}
"""


GRCOV_ENV = os.environ.copy()
GRCOV_ENV["CARGO_INCREMENTAL"] = "0"
GRCOV_ENV["RUSTFLAGS"] = (
    "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code "
    + "-Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
)
GRCOV_ENV["RUSTDOCFLAGS"] = "-Cpanic=abort"
GRCOV_ENV["PROPTEST_CASES"] = "100000"

SRC_FILE = "src/main.rs"
CARGO_TOML_FILE = "Cargo.toml"

if __name__ == "__main__":
    raw = None
    with open(SRC_FILE, "r") as finput:
        raw = finput.read()

    if raw is not None:
        (prefix, suffix) = split_prefix_suffix(raw)
        body = get_body(suffix)
        arg_ty_pairs = get_arg_ty_pair(suffix)
        proptest_code = codegen_proptest(body, arg_ty_pairs)
        print(proptest_code)

        with open(SRC_FILE, "w") as foutput:
            foutput.write(prefix + proptest_code)
        subprocess.run(
            ["sed", "-i", 's/^bolero.*/proptest = "1.0.0"/g', CARGO_TOML_FILE]
        )
        subprocess.run(
            ["sed", "-i", "s/rwasm-bolero/rwasm-proptest/g", CARGO_TOML_FILE]
        )
        subprocess.run(
            "timeout --signal=KILL 2m cargo test",
            shell=True,
            env=GRCOV_ENV,
        )
        subprocess.run(
            "grcov ../../../target/debug -s . --binary-path ../../../target/debug "
            + "-t lcov --branch --keep-only src/main.rs --ignore-not-existing -o coverage.lcov"
        )
