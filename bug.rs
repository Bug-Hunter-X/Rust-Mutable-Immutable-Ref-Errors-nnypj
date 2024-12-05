fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    // The following line will cause a compile-time error
    // because z is an immutable reference and we're trying
    // to modify x through z
    //*z += 1; 
}