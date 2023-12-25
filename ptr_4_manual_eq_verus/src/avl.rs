//! avl.rs --- Equivalence checking for `avl_rotate` and `avl_insert` translations.
use vstd::prelude::*;

verus! {

#[derive(PartialEq, Eq, Clone)]
struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32,
}

spec fn spec_height(maybe_node: &Option<Box<Node>>) -> int
    decreases maybe_node,
{
    match maybe_node {
        Some(n) => 1 + vstd::math::max(spec_height(&n.left), spec_height(&n.right)),
        None => 0,
    }
}

spec fn shape_eq(l: &Node, r: &Node) -> bool
    decreases l  //can be r , , ,
{
    l.key == r.key && l.height == r.height && (match (l.left, r.left) {
        (None, None) => true,
        (Some(ll), Some(rl)) => spec_eq(&ll, &rl),
        _ => false,
    }) && (match (l.right, r.right) {
        (None, None) => true,
        (Some(lr), Some(rr)) => spec_eq(&lr, &rr),
        _ => false,
    })
}

const HEIGHT_LIMIT: i32 = 100;

/// This function is a hack to suppress overflow concerns by
/// setting the heights to be below a threshold.
spec fn low_height(maybe_node: &Option<Box<Node>>) -> (ret: bool)
    decreases maybe_node,
{
    match maybe_node {
        Some(n) => n.height < HEIGHT_LIMIT && low_height(&n.left) && low_height(&n.right),
        None => true,
    }
}

proof fn low_height_recurses(maybe_node: &Option<Box<Node>>)
    requires
        low_height(maybe_node),
    ensures
        maybe_node.is_some() ==> low_height(&maybe_node.unwrap().left),
    maybe_node.is_some() ==> low_height(&maybe_node.unwrap().right),
    decreases maybe_node
{
}

spec fn right_height(maybe_node: &Option<Box<Node>>) -> (ret: bool)
    decreases maybe_node,
{
    match maybe_node {
        Some(n) => n.height == spec_height(maybe_node) && right_height(&n.left) && right_height(
            &n.right,
        ),
        None => true,
    }
}

fn llm_right_rotate(mut y: Box<Node>) -> Box<Node>
    requires
        y.left.is_some(),
        right_height(&Some(y)),
        low_height(&Some(y)),
{
    assert(low_height(&y.left)) by {
        low_height_recurses(&Some(y));
    }
    let mut x = y.left.take().unwrap();
    assert(low_height(&Some(x)));
    assert(low_height(&x.right)) by {
     low_height_recurses(&Some(x));
    }
    let mut t2 = x.right.take();
    assert(low_height(&t2));
    y.left = t2;
    assert(low_height(&Some(y)));
    //y.height = max(height(&y.left), height(&y.right)) + 1;
    x.right = Some(y);
    assert(low_height(&x.right));
    x.height = max(height(&x.left), height(&x.right)) + 1;
    x
}

fn crown_right_rotate(mut y: Box<Node>) -> Box<Node>
    requires
        y.left.is_some(),
        right_height(&Some(y)),
        y.height < i32::MAX,
{
    let mut x = y.left.take().unwrap();
    let mut T2 = x.right.take();
    if let Some(t2) = T2.as_ref() {
        assert(t2.height < i32::MAX - 2);
    }
    y.left = T2;
    y.height = max(height(&y.left), height(&y.right)) + 1 as i32;
    x.right = Some(y);
    x.height = max(height(&x.left), height(&x.right)) + 1 as i32;
    return x;
}

fn height(node: &Option<Box<Node>>) -> (ret: i32)
    requires
        low_height(node),
    ensures
        ret < HEIGHT_LIMIT,
{
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

fn max(a: i32, b: i32) -> (ret: i32)
    ensures
        b < a ==> ret == a,
        b >= a ==> ret == b,
{
    if b < a {
        a
    } else {
        b
    }
}

} // verus!
// Verus!
