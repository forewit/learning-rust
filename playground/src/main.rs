fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    impl IpAddr {
        // define a function for the enum
        fn to_string(&self) -> String {
            match self {
                IpAddr::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
                IpAddr::V6(s) => s.to_string(),
            }
        }
    }

    let v4 = IpAddr::V4(192, 168, 1, 100);
    let v6 = IpAddr::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));

    println!("v4: {}", v4.to_string());
    println!("v6: {}", v6.to_string());
}
