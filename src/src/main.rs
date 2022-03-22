mod dice;
use dice::Dice;
fn main() {
    let mut roller: dice::Roller = dice::Roller::new(Box::new(dice::Hc128RngEngine::new()));
    let dice: Dice = roller.roll_dice(2);
    println!("Das Würfelergebnis ist: {} von {}", dice.value, dice.sides);
}
