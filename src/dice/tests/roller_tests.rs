#[cfg(test)]
mod roller_tests {
    use crate::dice::MockRngEngine;
    use crate::dice::{Dice, Roller};
    use mockall::predicate::*;
    use rstest::rstest;

    #[test]
    fn roll_dice_should_return_dice_wtih_value() {
        let mut mock: MockRngEngine = MockRngEngine::new();
        mock.expect_next().return_const(1u32);
        let mut roller: Roller = Roller::new(Box::new(mock));

        let result: Dice = roller.roll_dice(3);
        assert_eq!(1, result.value);
    }

    #[rstest]
    #[case(1)]
    #[case(4)]
    #[case(8)]
    fn roll_dice_should_call_next_with_correct_range(#[case] sides: u32) {
        let mut mock: MockRngEngine = MockRngEngine::new();
        mock.expect_next()
            .with(eq(1), eq(sides))
            .times(1)
            .return_const(3u32);
        let mut roller: Roller = Roller::new(Box::new(mock));

        roller.roll_dice(sides);
    }

    #[rstest]
    #[case(5)]
    #[case(9)]
    #[case(12)]
    fn roll_dice_should_return_dice_with_value_from_rng_engine(#[case] value: u32) {
        let mut mock: MockRngEngine = MockRngEngine::new();
        mock.expect_next().return_const(value);
        let mut roller: Roller = Roller::new(Box::new(mock));

        let dice = roller.roll_dice(10);
        assert_eq!(value, dice.value);
    }

    #[rstest]
    #[case(2)]
    #[case(5)]
    #[case(18)]
    fn roll_dice_should_return_dice_wtih_same_sides_as_parameter(
        #[case] sides: u32,
    ) {
        let mut mock: MockRngEngine = MockRngEngine::new();
        mock.expect_next().return_const(1u32);
        let mut roller: Roller = Roller::new(Box::new(mock));
        let result: Dice = roller.roll_dice(sides);
        assert_eq!(sides, result.sides);
    }
}
