fn do_something() {}

fn main() {
    for _i in 1..100 {
        do_something();
    }

    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
