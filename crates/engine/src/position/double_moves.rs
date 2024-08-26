use crate::position::{Position, MOVES_CAPACITY, X_BAR};
use std::cmp::{max, min};

impl Position {
    /// Returns a vector of all possible moves when rolling a double.
    #[inline]
    pub(super) fn all_positions_after_double_move(&self, die: usize) -> Vec<Position> {
        self.double_moves_after_entering(die)
    }


    /// Returns a vector of all possible moves after entering the checkers from the bar.
    /// It takes into account the number of already entered checkers.
    fn double_moves_after_entering(
        &self,
        die: usize,
    ) -> Vec<Position> {
        let nr_movable_checkers = self.number_of_movable_checkers(die);
        if nr_movable_checkers == 0 {
            return vec![*self];
        }
        let mut moves: Vec<Position> = Vec::with_capacity(MOVES_CAPACITY);
        (self.smallest_pip_to_check(die)..X_BAR).for_each(|i1| {
            if self.can_move_when_bearoff_is_legal(i1, die) {
                let pos = self.clone_and_move_single_checker(i1, die);
                if nr_movable_checkers == 1 {
                    moves.push(pos);
                    return;
                }
                (pos.smallest_pip_to_check(die)..i1 + 1).for_each(|i2| {
                    if pos.can_move_when_bearoff_is_legal(i2, die) {
                        let pos = pos.clone_and_move_single_checker(i2, die);
                        if nr_movable_checkers == 2 {
                            moves.push(pos);
                            return;
                        }
                        (pos.smallest_pip_to_check(die)..i2 + 1).for_each(|i3| {
                            if pos.can_move_when_bearoff_is_legal(i3, die) {
                                let pos = pos.clone_and_move_single_checker(i3, die);
                                if nr_movable_checkers == 3 {
                                    moves.push(pos);
                                    return;
                                }
                                (pos.smallest_pip_to_check(die)..i3 + 1).for_each(|i4| {
                                    if pos.can_move_when_bearoff_is_legal(i4, die) {
                                        let pos = pos.clone_and_move_single_checker(i4, die);
                                        moves.push(pos);
                                    }
                                });
                            }
                        });
                    }
                });
            }
        });
        moves
    }

    /// Will return 4 if 4 or more checkers can be moved.
    /// Will return 0 if no checker can be moved.
    ///
    /// If for example `number_of_entered_checkers` is 1, the maximum return value is 3.
    fn number_of_movable_checkers(&self, die: usize) -> u32 {
        let mut number_of_checkers = 0;
        let mut pip = 24;
        let mut position = *self;
        let max_return_value = 4;

        // non bear off moves
        while number_of_checkers < max_return_value && pip > die {
            if position.can_move_when_bearoff_is_legal(pip, die) {
                let number_to_move = position.pips[pip];
                position.pips[pip] = 0;
                position.pips[pip - die] += number_to_move;
                number_of_checkers += number_to_move as u32;
            }
            pip -= 1;
        }
        if number_of_checkers >= max_return_value {
            return max_return_value;
        }
        // bear off moves
        return match position.pips.iter().rposition(|&p| p > 0) {
            None => number_of_checkers,
            Some(biggest_pip) => {
                let bearoff_checkers: u32 = if biggest_pip > 6 {
                    0 // no bearoff possible
                } else if biggest_pip > die {
                    // Only exact bearoff possible
                    // We need the `max` in case there are opponent's checkers on the pip.
                    max(0, position.pips[die]) as u32
                } else {
                    // no checkers on bigger pips, all bearoffs legal
                    position.pips[1..die + 1]
                        .iter()
                        .filter(|&&p| p > 0)
                        .sum::<i8>() as u32
                };
                number_of_checkers += bearoff_checkers;
                min(number_of_checkers, max_return_value)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::pos;

    #[test]
    fn bearoff_4_or_bearoff_less() {
        // Given
        let position = pos!(x 4:1, 3:1, 2:4; o 22:2);
        // When
        let resulting_positions = position.all_positions_after_double_move(2);
        // Then
        let expected1 = pos!(x 2:3, 1:1; o 22:2);
        let expected2 = pos!(x 3:1, 2:2; o 22:2);
        let expected3 = pos!(x 4:1, 2:1, 1:1; o 22:2);
        let expected4 = pos!(x 4:1, 3:1; o 22:2);
        assert_eq!(
            resulting_positions,
            vec![expected4, expected3, expected2, expected1],
        );
    }


}

#[cfg(test)]
mod private_tests {
    use crate::pos;

    #[test]
    fn number_of_movable_checkers_when_completely_blocked() {
        // Given
        let position = pos!(x 20:2; o 16:2);
        // When
        let actual = position.number_of_movable_checkers(4, 0);
        // Then
        assert_eq!(actual, 0);
    }

    #[test]
    fn number_of_movable_checkers_when_many_moves_would_be_possible() {
        // Given
        let position = pos!(x 20:2; o 16:1);
        // When
        let actual = position.number_of_movable_checkers(4, 0);
        // Then
        assert_eq!(actual, 4);
    }

    #[test]
    fn number_of_movable_checkers_when_blocked_after_one_move() {
        // Given
        let position = pos!(x 20:2; o 12:2);
        // When
        let actual = position.number_of_movable_checkers(4, 0);
        // Then
        assert_eq!(actual, 2);
    }
}
