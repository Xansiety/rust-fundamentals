fn main() {
    // Vectors
    // Vectors are dynamic and can grow or shrink in size
    let vector_items: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("The vector is: {:?}", vector_items);

    let mut vector_items_2 = Vec::new();

    // For loop to add items to the vector
    for i in 1..10 {
        vector_items_2.push(i);
    }

    println!("The vector dynamic is: {:?}", vector_items_2);
}
