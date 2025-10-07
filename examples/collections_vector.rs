fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);

    let third = &v[2];
    println!("The third element is {third}");

    let fourth = v.get(3);
    match fourth {
        Some(value) => println!("The fourth element is {value}"),
        None => println!("There is no fourth element"),
    }

    //-----------------------

    let first = &v[0];

    // v.push(7); //mutable borrow occurs
    // vectors put the values next to each other in memory,
    // adding a new element onto the end of the vector might
    // require allocating new memory and copying the old elements to the new space

    println!("The first element is: {first}");
}
