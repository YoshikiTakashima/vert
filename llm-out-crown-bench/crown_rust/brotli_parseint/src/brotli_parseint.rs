
fn parse_int(s: &str, low: i32, high: i32, result: &mut i32) -> bool {
    let mut value = 0;
    let mut i = 0;
    while i < 5 {
        let c = s.chars().nth(i).unwrap();
        if c == '\0' { break; }
        if !(c >= '0' && c <= '9') { return false; }
        value = value * 10 + (c as u32 - '0' as u32) as i32;
        i += 1;
    }
    if i == 0 { return false; }
    if i > 1 && s.starts_with('0') { return false; } 
    if s[i..].chars().next().is_some() { return false; } 
    if value < low || value > high { return false; }
    true
}
fn main(){
}