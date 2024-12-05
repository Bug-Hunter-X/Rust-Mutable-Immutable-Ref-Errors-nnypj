fn main() {
    let mut x = 5;
    let y = &mut x; // Mutable reference

    *y += 1; // Modify through mutable reference
    println!("x = {}", x); // Output: x = 6

    // Correct way to handle immutable reference if modification isn't needed:
    let z = &x;  
    println!("x (immutable ref) = {}", *z); //access through immutable ref
}
