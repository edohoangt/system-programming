// === Example 1 ===
// fn main() {
//     let mut s = String::from("hello");
//     let ref1 = &s; // shared ref to s
//     let ref2 = &ref1; // shared ref to ref1
//     let ref3 = &ref2;
//     s = String::from("goodbye");
//     println!("{}", s.to_uppercase()); // borrowed
// }

// === Example 2 ===
// fn drip_drop() -> String {
//     let s = String::from("hello world!");
//     return s;
// }

// fn main() {
//     println!("{}", drip_drop());
// }

// === Example 3 ===
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: &String = &v[0];
    println!("{}", s2);
}
