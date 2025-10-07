fn main() {
    let data = "initial contents";
    let s = data.to_string();

    let i = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    //------------------------------------------
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); //format! macro uses references 

    //------------------------------------------
    for c in "Зд".chars() {
        println!("{c}");
    }

    //------------------------------------------
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // can’t add two String values together, deref coercion: &s2 -> &s2[..]
    // s1 has been moved here and can no longer be used
    let s3 = s1 + &s2;
}
