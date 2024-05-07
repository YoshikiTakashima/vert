
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_key(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(key.as_bytes());
    hasher.finish()
}
fn main(){
}