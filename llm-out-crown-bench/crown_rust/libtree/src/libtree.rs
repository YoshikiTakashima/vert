
use std::fmt::Write;

fn utoa(string: &mut String, value: usize) {
    let mut n = value;
    let mut tmp = String::new();
    while n > 0 {
        tmp.write_fmt(format_args!("{}", n % 10)).unwrap();
        n /= 10;
    }
    string.write_str(&tmp.chars().rev().collect::<String>()).unwrap();
}

fn is_ascending_order(values: &[usize]) -> bool {
    for i in 1..values.len() {
        if values[i - 1] >= values[i] {
            return false;
        }
    }
    true 
}
fn main(){
}