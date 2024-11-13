use std::io::{self, Write};

fn main() {
    let mut tries: u64 = 1;

    loop {
        let mut input = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de l'entrée");
        //println!("{}", input.trim());
        if input.trim() == "The letter e" {
            println!("Number of trials: {}", tries);
            break;
        } else {
            tries += 1;
        }
    }    
}