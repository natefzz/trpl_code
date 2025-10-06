fn main() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: {:?}", home);
    println!("Loopback IP: {:?}", loopback);

    //-------------------------------------------
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
}
