# Manual Equivalence Checking

The VERT infrastructure is flexible with regard to the verifier
used. While large-scale evaluations generally require fully automatic
verifiers like Kani or Bolero, a smaller case study can use more
effort-heavy verifiers like Verus. This small case study uses Verus to
check equivalence where Kani failed.

## Benchmarks:
The benchmark consists of 4 programs from the CROWN benchmark set
where Kani failed to check equivalence.

- [ ] AVL: `src/avl.rs`
  - [ ] `avl_insert`
  - [ ] `avl_rotate`
- [ ] `brotli_parseint`: `src/parseint.rs`
- [x] `libtree`: `src/libtree.rs`

The files contain 2 functions per benchmark (`llm_` and `crown_`) and support code
to prove equivalence between the two.
