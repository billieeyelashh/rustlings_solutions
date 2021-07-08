// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!


fn main() {
    let a: [i32; 105] = [1,2,3,4,5,6,7,8,9,11,12,12,12,12,12,1,2,12,1,2,12,12,1,2,12,1,12,1,2,12,12,1,2,2,2,11,2,121,2,1,2,12,1,2,1,2,12,21,12,1,2,1,2,1,2,12,12,1,2,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
