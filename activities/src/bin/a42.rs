// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct Score {
    amount: usize,
    progression: usize,
    bonus: usize,
}

impl Score {
    fn new() -> Self {
        Self {
            amount: 0,
            progression: 1,
            bonus: 0,
        }
    }

    fn set_bonus(&mut self, x: usize) {
        self.bonus = x;
    }
}

impl Iterator for Score {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.amount += self.progression + self.bonus;
        return Some(self.amount);
    }
}

fn main() {
    let mut score = Score::new();
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    score.set_bonus(3);
    println!("{:?}", score.next());
    println!("{:?}", score.next());
    println!("{:?}", score.next());
}
