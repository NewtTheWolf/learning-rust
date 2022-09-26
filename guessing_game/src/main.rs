use std::io;

fn main() {
    println!("Rate die Zahl!");

    println!("Bitte gib deine Schätzung ein.");

    let x = "y";
    let mut y = "x";

    y = 1;

    println!("{y}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Zeile");

    println!("Du hast geschätzt: {guess}")
}
