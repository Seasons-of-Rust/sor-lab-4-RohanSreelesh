use std::{cmp::Ordering, fmt};

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult:first_shop_wins:Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        let mut first_shop_wins = 0;
        let mut second_shop_wins = 0;
        for card_from_shop_one in &self.cards {
            for card_from_shop_two in &other.cards {
                let current_result = card_from_shop_one.fight(card_from_shop_two);
                match current_result {
                    FightResult::Win => first_shop_wins += 1,

                    FightResult::Loss => second_shop_wins += 1,
                    _ => (),
                }
            }
        }

        // match first_shop_wins > second_shop_wins{
        //     true=>FightResult::Win,
        //     false=> match first_shop_wins == second_shop_wins {
        //         true=> FightResult::Tie,
        //         false=> FightResult::Loss,
        //     }
        // }

        //trying with cmp instead of nested match
        match first_shop_wins.cmp(&second_shop_wins) {
            Ordering::Greater => FightResult::Win,
            Ordering::Less => FightResult::Loss,
            Ordering::Equal => FightResult::Tie,
        }
    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}
