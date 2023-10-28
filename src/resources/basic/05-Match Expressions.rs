fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn main() {
    // Match Expressions

    let mut total: i32 = add(50, 50);
    let mut free_shipping: bool = true;

    if total > 50 {
        println!("You qualify for free shipping!");
        free_shipping = true;
    } else if total < 50 {
        println!("If you add more items, you qualify for free shipping!");
    } else {
        println!("add something to your cart!");
    }

    match free_shipping {
        true => total = total + 0,
        false => total = total + 10,
    }

    match total {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match!"),
    }

    println!("Your total is: ${:?}", total);
}
