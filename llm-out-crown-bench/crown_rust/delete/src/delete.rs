
use std::cmp::max;

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32, 
}

fn get_balance(node: &Node) -> i32 {
    match node.left.as_ref() {
        None => 0,
        Some(left) => left.height - match node.right.as_ref() {
            None => 0,
            Some(right) => right.height,  
        },
    }
}

fn left_rotate(node: &mut Node) -> &mut Node {
    let y = node.right.take().unwrap();
    let t2 = y.left.take();

    node.right = t2;
    node.height = max(match node.left.as_ref() {
        Some(left) => left.height,
        None => 0,  
    }, match node.right.as_ref() {
        Some(right) => right.height,
        None => 0,
    }) + 1;

    y.left = Some(node.to_owned());
    y.height = max(match y.left.as_ref() {
        Some(left) => left.height,
        None => 0,  
    }, match y.right.as_ref() {
        Some(right) => right.height,
        None => 0,
    }) + 1;
    y 
}

fn right_rotate(node: &mut Node) -> &mut Node {
    let x = node.left.take().unwrap();
    let t2 = x.right.take();

    node.left = t2;
    node.height = max(match node.left.as_ref() {
        Some(left) => left.height,
        None => 0,  
    }, match node.right.as_ref() {
        Some(right) => right.height,
        None => 0,
    }) + 1; 

    x.right = Some(node.to_owned());
    x.height = max(match x.left.as_ref() {
        Some(left) => left.height,
        None => 0,  
    }, match x.right.as_ref() {
        Some(right) => right.height,
        None => 0,
    }) + 1;
    x
}

fn delete_node(node: &mut Option<Box<Node>>, key: i32) {
    match node {
        None => (),
        Some(n) => {
            if key < n.key {
                delete_node(&mut n.left, key);
            } else if key > n.key {
                delete_node(&mut n.right, key);     
            } else {
                match (n.left.as_ref(), n.right.as_ref()) {
                    (None, None) => *node = None,
                    (None, right) => *node = n.right.take(), 
                    (left, None) => *node = n.left.take(),  
                    (Some(_), Some(_)) => {
                        let min_node = min_value_node(n.right.as_mut().unwrap());
                        n.key = min_node.key;
                        delete_node(&mut n.right, min_node.key);
                    }
                }
            }
            if let Some(n) = node.as_mut() {
                n.height = 1 + max(match n.left.as_ref() {
                    Some(left) => left.height,
                    None => 0,  
                }, match n.right.as_ref() {
                    Some(right) => right.height,
                    None => 0,
                }); 
                let balance = get_balance(n);
                
                if balance > 1 && get_balance(n.left.as_ref().unwrap()) >= 0 {
                } else if balance > 1 && get_balance(n.left.as_ref().unwrap()) < 0 { 
                    n.left = Some(left_rotate(n.left.as_mut().unwrap()));
                } else if balance < -1 && get_balance(n.right.as_ref().unwrap()) <= 0 {
                } else if balance < -1 && get_balance(n.right.as_ref().unwrap()) > 0 {
                    n.right = Some(right_rotate(n.right.as_mut().unwrap()));    
                }
            }
        }
    }
}
fn main(){
}