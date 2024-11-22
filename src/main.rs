use std::io;
fn number_determinant(value: i32) -> &'static str {
    if value == 0 || value % 2 == 0 {
        "even"
    } else {
        "odd"
    }
}

fn main() {
    println!("Write the value: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to write in input. ");
    let value: i32 = input.trim().parse().expect("Please write a volid number. ");

    let result = number_determinant(value);

    println!("This is your value: {input}. And it is difaned as: {result}. ");
}
