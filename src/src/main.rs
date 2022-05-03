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
    io::stdin()
        .read_line(&mut input)
        .expect("Keine Eingabe der Würfelart");

    //Eingabebearbeitung
    let mut parse_result: Result<u32, ParseIntError> = input.trim().parse::<u32>();
    match parse_result {
        Ok(number) => {
            let mut roller: Roller = Roller::new(Box::new(Hc128RngEngine::new()));
            print!("Würfelanzahl eingeben: ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Keine Eingabe der Anzahl");
            parse_result = input.trim().parse::<u32>();

            match parse_result {
                Ok(count) => {
                    println!("Du würfelst {} {} seitige Würfel", count, number);
                    let mut x = 0;
                    let mut sum = 0;
                    while x < count {
                        let dice: Dice = roller.roll_dice(number);
                        println!("Das Würfelergebnis ist: {} von {}", dice.value, dice.sides);
                        sum = sum + dice.value;
                        x = x + 1;
                    }
                    println!("Die Augensumme beträgt {}", sum);
                }
                Err(_countfail) => println!("Würfelanzahl falsch"),
            }
        }
        Err(_fail) => println!("Wieviele numerische Seiten hat der Würfel, du Pfogel?"),
    }
}
