fn main() {
    another_function(5, 6);

    // This is a statement
    let y = 6;

    // let x = (let y = 5);
    /***
     * The `let y = 6` statement does not return a value,
     * so there isn’t anything for x to bind to.
     * This is different from what happens in other languages,
     * such as C and Ruby, where the assignment returns the value of the assignment. 
     */

    let a = 5;
    let b = {
        let a = 3;
        a + 1
    };
    println!("The value of b is: {}", b);

    let z = plus_one(10);
    println!("The value of z is: {}", z);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}