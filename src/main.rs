use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("🔢 Devine le nombre ! (1 - 50) 🔢\n\n");

    let secret_number = rand::thread_rng().gen_range(1..=50);

    //println!("Le nombre mysthère est {}", secret_number);

    let mut turn = 0;

    loop {
        turn += 1;

        println!("Veuillez entrer un nombre :");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("❌ Échec de lecture de la ligne.\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\n❌ Veuillez entrer un nombre valide !\n");
                continue;
            }
        };

        //println!("Vous avez deviné : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("👇 Trop petit\n"),
            Ordering::Greater => println!("☝️ Trop grand\n"),
            Ordering::Equal => {
                println!("\n🎉 Vous avez gagné en {} tour(s) !", turn);
                break;
            }
        }
    }
}
