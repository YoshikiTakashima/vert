//! parseint.rs --- Equivalence checking for `brotli_parseint`.
use vstd::prelude::*;

verus! {

fn llm_is_ascending_order(values: &Vec<usize>) -> (ret: bool)
    ensures
        ret <==> (forall|a: int| 0 < a < values.len() ==> #[trigger] values[a - 1] < values[a]),
{
    let mut i: usize = 1;
    while i < values.len()
        invariant
            1 <= i,
            forall|b: int| 0 < b < i ==> #[trigger] values[b - 1] < values[b],
    {
        if values[i - 1] >= values[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn crown_is_ascending_order(v: &Vec<usize>) -> (ret: bool)
    ensures
        ret <==> (forall|a: int| 0 < a < v.len() ==> #[trigger] v[a - 1] < v[a]),
    {
    let mut j: usize = 1;
    while j < v.len()
        invariant
            1 <= j,
            forall|b: int| 0 < b < j ==> #[trigger] v[b - 1] < v[b],
    {
        let left: usize = v[j.wrapping_sub(1usize)];
        let right: usize = v[j];
        if left >= right {
            return false;
        }
        j = j.wrapping_add(1);
    }
    return true;
}

fn call_both(values: &Vec<usize>) {
    let a = llm_is_ascending_order(values);
    let b = crown_is_ascending_order(values);
    assert(a == b);
}

} // verus!
