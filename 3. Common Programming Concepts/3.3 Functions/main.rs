fn main() {
    let x = another_function(5);

    println!("The value of x is: {x}");
}

fn another_function(num_to_print: i32) -> i32 {
    println!("The num is: {num_to_print}");

    return num_to_print;
}
