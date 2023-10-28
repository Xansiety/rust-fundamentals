fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    // Source: https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types

    // Rust is a statically typed language, which means that it must know the types of
    //all variables at compile time.

    //! Number type not exist in Rust, all is represented by i32, i64, u32, u64, f32, f64
    // let foo: i32 = 50;
    // let bar: i32 = 50;
    let food: i32 = add(50, 50);

    println!("food: {}", food);
}
