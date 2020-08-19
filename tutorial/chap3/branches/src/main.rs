fn main() {
    let number = 6;

    // It’s also worth noting that the condition in this code must be a bool.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression, we can use it on the right side of a let statement.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    /***
     * Each arm of the if must be the same type.
     * In the following example, we’ll get an error.
     */
    // let number = if condition { 5 } else { "six" };
    println!("The value of number is: {}", number);

    /***
     * Returning Values from Loops
     */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    /***
     * Conditional Loops with while
     */
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    /***
     * Looping Through a Collection with for
     */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    /***
     * Range is a type provided by the standard library that generates
     * all numbers in sequence starting from one number and ending before another number.
     * rev() is to reverse the range.
     */
    for num in (1..4).rev() {
        println!("{}!", num);
    }
}
