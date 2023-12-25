//! parseint.rs --- Equivalence checking for `brotli_parseint`.
//!
//!

use vstd::prelude::*;

verus! {
    spec fn exp(i: int) -> int
        decreases i
    {
        if i > 0 {
            10 * exp(i -1)
        } else {
            1
        }
    }

    proof fn exp_monotone_by_one(v: u32, a: int)
        requires
        v < exp(a)
        ensures
        v < exp(a+1)
    {}

    proof fn exp_monotone_any(v: u32, a: int, b: int)
        requires
        a < b,
    v < exp(a)
        ensures
        v < exp(b)
        decreases b - a
    {
        if b == a + 1 {
            assert(v < exp(b)) by {
                exp_monotone_by_one(v, a);
            };
        } else {
            assert(v < exp(a + 1)) by {
                exp_monotone_by_one(v, a);
            };
            exp_monotone_any(v, a+1, b);
        }
    }

// fn crown_parseint(
//     mut s: &Vec<char>,
//     mut low: i32,
//     mut high: i32,
// ) -> Option<i32> {
//     let mut value = 0i32;
//     let mut i: usize = 0;
//     while i < 5 {
//         let mut c: char = s[i];
//         if c == '\0' {
//             break;
//         }
//         if (c as i32) < ('0' as i32)
//             || (c as i32) > ('9' as i32)
//         {
//             return None;
//         }
//         value= 10 * value + (c as i32 - '0' as i32);
//         i+= 1;
//     }
//     if i == 0 {
//         return None;
//     }
//     if i > 1
//         && s[0] == '\0'
//     {
//         return None;
//     }
//     if value < low || value > high {
//         return None;
//     }
//     return Some(value);
// }

    fn llm_parseint(s: &Vec<char>, low: u32, high: u32) -> (ret: Option<u32>)
        requires
        valid_vector(s)
    {
        let mut value = 0;
        let mut i = 0;
        assert(value < exp(0));
        while i < 5
            invariant
            valid_vector(s),
        value < exp(i as int)
        {
            let c = s[i];
            if c == '\0' { break; }
            if !(c as u32 >= '0' as u32 && c as u32 <= '9' as u32) { return None; }
            assert(i < 6);
            assert(value < exp(6)) by {
                exp_monotone_any(value, i as int, 6);
            };
            assert(exp(6) == 1000000) by (compute_only);
            value = value * 10 + (c as u32 - '0' as u32) as u32;
            i += 1;
        }
        if i == 0 { return None; }
        if i > 1 && s[0] ==  '\0' { return None; }
        if s.len() > 5 { return None; }
        if value < low || value > high { return None; }
        Some(value)
    }

    spec fn valid_vector(s: &Vec<char>) -> bool {
        s.len() == 5 &&
            (('1' as u32) <=  s[0] as u32 <= ('9' as u32)) &&
            (('1' as u32) <=  s[1] as u32 <= ('9' as u32)) &&
            (('1' as u32) <=  s[2] as u32 <= ('9' as u32)) &&
            (('1' as u32) <=  s[3] as u32 <= ('9' as u32)) &&
            (('1' as u32) <=  s[4] as u32 <= ('9' as u32))
    }

} // verus!
