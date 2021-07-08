// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!
// Bei impl koennen functionen auf structs draufgepackt werden
// Enums sind aehnlich zu structs.
// Wann sollte man sie benutzen?
//

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
