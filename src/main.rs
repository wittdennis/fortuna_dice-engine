mod dice;
use dice::Dice;
use std::io;
use std::io::Write;
fn main() {
    //Input of Number
    print!("Seitenzahl eingeben: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Keine Eingabe");
    println!("Du würfelst einen {} seitigen Würfel", input.trim());
    let number: u32 = input.trim().parse().unwrap();

    let mut roller: dice::Roller = dice::Roller::new(Box::new(dice::Hc128RngEngine::new()));
    let dice: Dice = roller.roll_dice(number);
    println!("Das Würfelergebnis ist: {} von {}", dice.value, dice.sides);
}