// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)


fn main() {
    let mut number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    { // This is called shadowing creating a new scope with {} and the variable can be assigned a new type 
    let number =3;
    println!("Number plus two is : {}", number + 2);
    }
}