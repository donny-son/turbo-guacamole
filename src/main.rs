use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("GUESS THE NUMBER!!!!!!!!!!!!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Pleasure input your guess(1 ~ 100)");

    loop {

        let mut guess = String::new();

        print!("👉 ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small..."),
            Ordering::Greater => println!("TOO BIG!!!"),
            Ordering::Equal => {
                println!("!🎉!🎉YOU WIN!🎉!🎉");
                break;
            },
        }
    }
}
