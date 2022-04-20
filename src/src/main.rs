mod dice;
use dice::{Dice, Hc128RngEngine, Roller};
use std::io;
use std::io::Write;
use std::num::ParseIntError;

fn main() {
    //Input of Number & Answer
    print!("Seitenzahl eingeben: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Keine Eingabe");

    //Eingabebearbeitung
    let parse_result: Result<u32, ParseIntError> = input.trim().parse::<u32>();
    match parse_result {
        Ok(number) => {
            let mut roller: Roller = Roller::new(Box::new(Hc128RngEngine::new()));
            let dice: Dice = roller.roll_dice(number);
            println!("Du würfelst einen {} seitigen Würfel", number);
            println!("Das Würfelergebnis ist: {} von {}", dice.value, dice.sides);
        }
        Err(_fail) => println!("Wieviele numerische Seiten hat der Würfel, du Pfogel?"),
    }
}
