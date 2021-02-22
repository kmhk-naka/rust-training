#[macro_use]
extern crate icecream;

/*
* https://www.youtube.com/watch?v=ClPrjjHmo2Y
*/

#[allow(unused_variables)]
fn main() {
    let example_str: &str = "John";
    let example_string: String = String::from("Partner");

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string;

    // let test = "first" + "second";
    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    // let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str("Some hardcoded literal");
    mut_string.push_str(example_str);
    mut_string.push('m');
    ic!(mut_string);

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;
    ic!(combined);

    let str_from_substring: &str = &example_str[0..2];
    let str_from_substring2: &str = &example_str[0..=2];
    ic!(str_from_substring);
    ic!(str_from_substring2);

    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(c) => ic!(c),
        None => {}
    }

    // Following is the shorthand of lines 42-47.
    if let Some(c) = example_str.chars().nth(2) {
        ic!(c);
    }
}
