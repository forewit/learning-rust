fn main() {
    println!("Hello, world!");

    // test each function
    println!("3 + 5 = {}", add(3, 5));
    countdown(4);
    the_twelve_days();
}

// add two numbers
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Blastoff countdown
fn countdown(n: i32) {
    for number in (1..=n).rev() {
        println!("{}", number);
    }

    println!("Blastoff!");
}


// print the lyrics to 'The Twelve Days of Christmas'
const PREFIXES: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];
const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree.",
    "two Turtle Doves,",
    "three French Hens,",
    "four Calling Birds,",
    "five Gold Rings,",
    "six Geese-a-Laying,",
    "seven Swans-a-Swimming,",
    "eight Maids-a-Milking,",
    "nine Ladies Dancing,",
    "ten Lords-a-Leaping,",
    "eleven Pipers Piping,",
    "twelve Drummers Drumming,"
];

fn the_twelve_days() {
    for i in 1..=12 {
        println!("On the {} day of Christmas my true love gave to me:", PREFIXES[i - 1]);
        
        for j in (1..=i).rev() {
            if i > 1 && j == 1 { print!("and ");}
            println!("{}", GIFTS[j - 1]);
        }
    }
}

