fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    // Conditional Statements and Operators
    // Operators Source: https://doc.rust-lang.org/book/appendix-02-operators.html

    let total: i32 = add(50, 50);

    if total > 50 {
        println!("Greater than 50");
    } else if total < 50 {
        println!("Less than 50");
    } else {
        println!("Equal to 50");
    }
}