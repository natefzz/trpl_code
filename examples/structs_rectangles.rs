#[derive(Debug)] //[1-1]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:?}", rect1); //[1] This will not work because Rectangle does not implement the Debug trait

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
