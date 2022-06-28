use std::fmt::{Display, Formatter};
use rand::prelude::*;
use crate::evolution::{CanMutate, Fitness};

#[derive(Debug, Clone)]
pub struct Creature {
    pub power: i8,
    pub toughness: i8,
    pub speed: i8,
}

impl Display for Creature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "P:{} T:{} S:{}", self.power, self.toughness, self.speed)
    }
}

impl Default for Creature {
    fn default() -> Self {
        Creature {
            power: thread_rng().gen_range(1..10),
            toughness: thread_rng().gen_range(1..10),
            speed: thread_rng().gen_range(1..10),
        }
    }
}

impl CanMutate for Creature {
    fn mutate(&mut self) {
        self.power += thread_rng().gen_range(-2i8..2i8);
        self.toughness += thread_rng().gen_range(-2i8..2i8);
        self.speed += thread_rng().gen_range(-2i8..2i8);
    }
}

impl Fitness for Creature {
    fn evaluate(&self) -> f32 {
        self.power as f32 * self.toughness as f32 + self.speed as f32 * self.toughness as f32
    }
}
