use std::rc::Rc;

struct Node {
    key: i32,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>, 
}

fn min_value_node(node: Rc<Node>) -> Rc<Node> {
    let mut current = node;
    while let Some(left) = current.left.as_ref() {
        current = left.clone();
    }
    current
}
fn main(){
}