fn main() {
    let some_bool: bool = true;
    let some_int: i32 = 30;

    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    match some_int {
        0 => println!("Hit 0 branch"),
        1..=100 => {
            println!("Between 1 and 100 branch");
            println!("Some more stuff");
        }
        _ => println!("Else branchA,")
    }

    let var_from_match = match some_bool {true => 10, false => 20};

    let var_from_match2 = match some_int {
        0 => 0,
        1 | 2 => 100, // use a single `|` to separate multiple alternative patterns
        _ => 200,
    };
}
