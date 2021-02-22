fn main() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let n = 15;
    match n {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Aint' special"),
    }

    let pair = (0, -2);
    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("No match"),
    }

    let pair2 = (5, -5);
    match pair2 {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }

    let p = 5;
    match p {
        n @ 1 ..= 12 => println!("n: {}", n),
        n @ 13 ..= 19 => println!("n: {}", n),
        _ => println!("No match"),
    }
}
