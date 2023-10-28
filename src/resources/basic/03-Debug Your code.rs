fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    // You can debug your code with macros

    // Macros support the following:
    // - Variadic arguments
    // - Pretty Syntax
    // - Metaprogramming

    let food: i32 = add(50, 50);

    // Println! is a macro, you can identify them with the ! at the end
    // println!("food: {} {}", food, true); // the placeholder its replaced with the value of the variable according to the order
    // println!("food: {0} {0}", food); // can use a position to repeat the value of the variable
    println!("{:?}", food); // {:?} is used to debug  a complex data structure
}
