fn main() {
    //  declare a variable -> let
    // Should be on snake_case
    //! Variables are immutable by default
    //* if you want to make it mutable, add mut keyword
    let my_name = "Rust";
    let mut my_mutable_name = "Rust";
    // my_name = "Python"; //! Error
    my_mutable_name = "WASM"; //? No Error

    print!("Hello, {}!", my_name);
    println!("Hello Mutable, {}!", my_mutable_name);
}
