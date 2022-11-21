fn variable_scope() {
    println!("a variable scope!");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s is: {}", s);
}

// fn double_free_error() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}, world!", s1);
// }

fn clone() {
    // heap data got copied, an expensive method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn ownership_and_function() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    // println!("{}", s);
    println!("{}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn transfer_ownership_return_values() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn return_ownership_in_tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // println!("s1: {}", s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn main() {
    println!("Hello, world!");
    variable_scope();
    clone();
    ownership_and_function();
    transfer_ownership_return_values();
    return_ownership_in_tuple();
}
