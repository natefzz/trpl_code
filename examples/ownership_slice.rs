fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..2]; // he
    let slice2 = &s[..2]; // he
    let slice3 = &s[..len]; // hello
    let slice4 = &s[..]; // hello

    println!("{slice}, {slice2}, {slice3}, {slice4}");

    //---------------------------------------

    let a = [1, 2, 3, 4, 5];

    let a_slice = &a[1..3]; // [2, 3]
}
