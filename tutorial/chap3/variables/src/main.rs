fn main() {
    /*
    * Constants can be declared in any scope,
    * including the global scope, which makes them
    * useful for values that many parts of code need to know about.
    */
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
    * Shadowing
    */
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);
    
    /*
    * This construct is allowed because the first spaces variable is a string type
    * and the second spaces variable, which is a brand-new variable that happens to have
    * the same name as the first one, is a number type.
    * Shadowing thus spares us from having to come up with different names,
    * such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. 
    */
    let spaces = "   ";
    let spaces = spaces.len();
    
    /*
    * If we try to use mut for this, as shown here, we’ll get a compile-time error:
    */
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
