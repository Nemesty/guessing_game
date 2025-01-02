use std::io;

fn main() {
    println!("Devine le nombre !");

    println!("Veuillez entrer un nombre :");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Échec de lecture de la ligne.");

    println!("Vous avez deviné : {}", guess);
}
