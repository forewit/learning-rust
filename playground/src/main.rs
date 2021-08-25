fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    // spaces is now "3" in this scope

    let mut spaces = "   ";
    let spaces = spaces.len();
    // this will not compile

    println!("{}", spaces);
}
