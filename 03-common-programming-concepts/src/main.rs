fn const_part() {
    println!("const part");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Three hours in second {}", THREE_HOURS_IN_SECONDS);
}

fn shadowing() {
    println!("shadowing part");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn shadowing_two() {
    println!("shadowing two");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is: {guess}");
}

fn statement_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is: {y}");
}

fn five() -> i32 {
    5
}

fn get_value_from_function() {
    let number = five();
    println!("number is: {number}");
}

fn main() {
    const_part();
    shadowing();
    shadowing_two();
    data_types();
    statement_expression();
    get_value_from_function();
}
