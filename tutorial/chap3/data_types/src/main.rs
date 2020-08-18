fn main() {
    /***
     * Type annotation
     */
    let guess: u32 = "42".parse().expect("Not a number!");
    /***
     * If we don’t add the type annotation here, Rust will display the error, which means
     * the compiler needs more information from us to know which type we want to use
     */
    // let guess = "42".parse().expect("Not a number!");

    /***
     * Numeric Operations
     */
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

    /***
     * The Character Type
     */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // https://doc.rust-lang.org/stable/std/convert/trait.Into.html#tymethod.into
    let heart_eyed_cat: u32 = heart_eyed_cat.into();
    // let heart_eyed_cat = heart_eyed_cat as u32;
    print!("The heart_eyed_cat value is: {}", heart_eyed_cat);

    /***
     * Compound Types
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {}", y);

    /***
     * we can access a tuple element directly by using a period (.)
     * followed by the index of the value we want to access. 
     */
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    /***
     * The Array Type
     */
    // let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];  // let a = [3, 3, 3, 3, 3];

    let first = a[0];   // 1
    let second = a[1];  // 2
}
