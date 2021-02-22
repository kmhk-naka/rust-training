#[allow(unused_variables)]
#[macro_use]
extern crate icecream;

fn main() {
    let returned_data = some_function(2.2, 50);
    ic!(returned_data);

    some_procedure(2.3, 1);

    let string_slic_var: &str = "Howdy!";
    some_str_procedure(string_slic_var);

    let string_var = String::from("I'm a REAl String :)");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    // some_string_procedure(string_var); // Compile Error
}

fn some_string_procedure(param: String) {
    ic!(param);
}

fn some_str_procedure(param: &str) {
    ic!(param);
}

fn some_procedure(param_a: f64, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

fn some_function(param_a: f64, param_b: i32) -> f64 {
    ic!();

    if param_a < 100. {
        let return_var = 10.1 * param_a + param_b as f64;

        return_var
    } else {
        -1.
    }
}
