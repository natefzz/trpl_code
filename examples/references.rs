// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // error: cannot borrow as mutable
// }

//-------------------------------------------------

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world"); // works
// }

//-------------------------------------------------

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time

//     println!("{r1}, {r2}");
// }

//-------------------------------------------------

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable

//     println!("{r1}, {r2}, and {r3}");
// }

//-------------------------------------------------
// dangling error
fn main() {
    let s = dangle();
    println!("{}", s);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // error: `s` does not live long enough
// }

fn dangle() -> String {
    let s = String::from("hello");
    s // return the String, not a reference to it
}
