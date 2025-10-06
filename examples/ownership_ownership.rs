fn main() {
    // let mut s = "hello";          // &str type, immutable
    let mut s = "hello".to_string(); //String type, mutable
    s.push_str(", world");

    // ------------------------------------------------

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); //value borrowed here after move

    // ------------------------------------------------

    let mut x1 = String::from("hello"); // drop old value when reassigned
    x1 = String::from("ahoy");
    println!("{}", x1); //prints "ahoy"
}
