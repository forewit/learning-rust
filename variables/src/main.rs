fn main() {
     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
 
     // remainder
     let remainder = 43 % 5;

     // boolean
     let is_true: bool = true;

     // emoji characters
     let cat: char = '🐱';

     // string
     let name: &str = "Miles";

     // tuples -- can have multiple types
     let tup: (i32, f64, u8) = (500, 2.0, 3);

     // destructuring tuples
     let (a, b, c) = tup;
     let a = tup.0;
     let b = tup.1;
     let c = tup.2;

     // array -- all must be the same type
     let array: [i32; 3] = [1, 2, 3];

     // array of same values
     let array2 = [3; 5]; // equals: [3, 3, 3, 3, 3]
}
