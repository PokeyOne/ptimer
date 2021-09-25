use std::io;

fn main() {
    println!("Please standby while we process your request.");
    println!("An agent will be with you shortly.");

    let mut blah = String::new();

    io::stdin()
        .read_line(&mut blah)
        .expect("Failed to read line");

    println!("Blah was {}", blah);
}
