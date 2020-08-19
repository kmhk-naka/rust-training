use std::io;

fn main() {
    println!("Please input a number grater than one.");

    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number grater than 1");

    let x = fibonacci_number(n);

    println!("The Fibonacci number is: {}", x);
}

fn fibonacci_number(n: u32) -> u32 {
    if n == 1 {
        return 0
    }else if n == 2 {
        return 1
    }

    fibonacci_number(n - 1) + fibonacci_number(n - 2)
}
