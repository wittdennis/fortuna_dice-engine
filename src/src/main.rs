mod dice;
use dice::{Dice, Hc128RngEngine, Roller};
use std::io;
use std::io::Write;
use std::num::ParseIntError;

fn main() {
    let side_count = get_numeral_input("Seitenzahl eingeben", "Seitenzahl nicht zugelassen");
    let dice_count = get_numeral_input("Würfelanzahl eingeben", "Würfelzahl nicht zählbar");
    let mut roller: Roller = Roller::new(Box::new(Hc128RngEngine::new()));
    println!("Du würfelst {} {} seitige Würfel", dice_count, side_count);
    let mut x = 0;
    let mut sum = 0;
    while x < dice_count {
        let dice: Dice = roller.roll_dice(side_count);
        println!("Das Würfelergebnis ist: {} von {}", dice.value, dice.sides);
        sum = sum + dice.value;
        x = x + 1;
    }
    println!("Die Augensumme beträgt {}", sum);
}
// function numeral I/O
fn get_numeral_input(input_text: &str, error_text: &str) -> u32 {
    let mut user_input: u32 = 0;
    //do while loop
    let mut x = 0;
    loop {
        print!("{}: ", input_text);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(error_text);
        let parse_result: Result<u32, ParseIntError> = input.trim().parse::<u32>();
        match parse_result {
            Ok(number) => user_input = number,
            Err(_) => println!("{}", error_text),
        }
        x = x + 1;

        if user_input > 0 {
            break;
        } else if x >= 3 {
            panic!("Anzahl der Fehlversuche zu hoch!");
        }
    }
    return user_input;
}