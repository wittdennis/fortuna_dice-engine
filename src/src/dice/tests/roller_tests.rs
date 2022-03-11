use crate::dice::Roller;
use rstest::rstest;

#[cfg(test)]
mod roller_tests {
    use super::*;

    #[test]
    fn roll_dice_should_return_dice_wtih_value_when_function_is_called() {
        let roller = Roller::new();

        let result = roller.roll_dice(3);
        assert_eq!(1, result.value);
    }

    #[rstest]
    #[case(2)]
    #[case(5)]
    #[case(18)]
    fn roll_dice_should_return_dice_wtih_same_sides_as_parameter_when_function_is_called(#[case] sides: u32) {
        let roller = Roller::new();

        let result = roller.roll_dice(sides);
        assert_eq!(sides, result.sides);
    }
}
