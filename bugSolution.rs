fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    let z = &x; // z is an immutable reference to x
    // Correctly using z without modification
    println!("x (via z) = {}", *z);
} 