use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use crate::evolution::{CanMutate, CanReproduce, FitnessMeasure};
use crate::Fitness;

#[derive(Debug, Clone, PartialEq)]
pub enum LinecraftTile {
    FriendlyBase,
    EnemyBase,
    Empty,
    Minerals,
    Gas,
    Swamp,
}

impl LinecraftTile {
    pub fn random() -> LinecraftTile {
        match thread_rng().gen_range(0..20) {
            0 => LinecraftTile::FriendlyBase,
            1 => LinecraftTile::EnemyBase,
            2..=8 => LinecraftTile::Empty,
            9..=13 => LinecraftTile::Minerals,
            14..=17 => LinecraftTile::Gas,
            18..=20 => LinecraftTile::Swamp,
            _ => LinecraftTile::Empty,
        }
    }

    pub fn resource_value(&self) -> f32 {
        match self {
            LinecraftTile::FriendlyBase => 0.0,
            LinecraftTile::EnemyBase => 0.0,
            LinecraftTile::Empty => 1.0,
            LinecraftTile::Minerals => 3.0,
            LinecraftTile::Gas => 3.0,
            LinecraftTile::Swamp => -1.0,
        }
    }
}

impl Display for LinecraftTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            LinecraftTile::FriendlyBase => "1",
            LinecraftTile::EnemyBase => "2",
            LinecraftTile::Empty => "_",
            LinecraftTile::Minerals => "*",
            LinecraftTile::Gas => "$",
            LinecraftTile::Swamp => "x",
        })
    }
}

#[derive(Debug, Clone)]
pub struct LinecraftMap {
    tiles: Vec<LinecraftTile>,
}

impl LinecraftMap {
    pub fn count_empty(&self) -> usize {
        self.tiles.iter().filter(|&t| *t == LinecraftTile::Empty).count()
    }

    pub fn count_friendly_base(&self) -> usize {
        self.tiles.iter().filter(|&t| *t == LinecraftTile::FriendlyBase).count()
    }

    pub fn count_enemy_base(&self) -> usize {
        self.tiles.iter().filter(|&t| *t == LinecraftTile::EnemyBase).count()
    }

    pub fn minimal_distance_between_bases(&self) -> usize {
        let friendly = self.tiles.iter().position(|t| *t == LinecraftTile::FriendlyBase);
        let enemy = self.tiles.iter().position(|t| *t == LinecraftTile::EnemyBase);

        if friendly.is_some() && enemy.is_some() {
            let friendly = friendly.unwrap();
            let enemy = enemy.unwrap();

            friendly.abs_diff(enemy)
        } else {
            0
        }
    }

    pub fn value_around_base(&self, base: LinecraftTile) -> f32 {
        let base_at = self.tiles.iter().position(|t| *t == base);

        if base_at.is_some() {
            let base_at = base_at.unwrap();
            self.tiles.iter().enumerate().map(|(index, tile)| {
                if base_at == index {
                    0.0
                } else {
                    let dist = base_at.abs_diff(index) as f32;
                    tile.resource_value() / dist
                }
            }).sum()
        } else {
            0.0
        }
    }
}

impl Default for LinecraftMap {
    fn default() -> Self {
        LinecraftMap {
            tiles: (1..10)
                    .map(|i| LinecraftTile::random())
                    .collect()
        }
    }
}

impl Display for LinecraftMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tiles.iter().fold(Ok(()), |result, next| {
            result.and_then(|_| write!(f, "{}", next))
        })
    }
}

impl CanMutate for LinecraftMap {
    fn mutate(&mut self) {
        let count_changes = thread_rng().gen_range(0..(self.tiles.len() / 2));
        let mut indices = (0..self.tiles.len()).collect::<Vec<_>>();
        indices.shuffle(&mut thread_rng());

        let change_indices = indices[0..count_changes].iter().collect::<Vec<_>>();;
        for index in change_indices {
            *self.tiles.get_mut(*index).unwrap() = LinecraftTile::random();
        }
    }
}

impl CanReproduce<LinecraftMap> for LinecraftMap {
    fn reproduce(&self, partner: &LinecraftMap) -> LinecraftMap {
        fn merge(a: &LinecraftTile, b: &LinecraftTile) -> LinecraftTile {
            use LinecraftTile::*;
            match (a, b) {
                (FriendlyBase, Empty) | (Empty, FriendlyBase) => FriendlyBase,
                (EnemyBase, Empty) | (Empty, EnemyBase) => EnemyBase,
                (a, b) if thread_rng().gen_bool(0.92) =>
                    if rand::random() { a.clone() } else { b.clone() },
                (a, b) =>
                    LinecraftTile::random()
            }
        }

        LinecraftMap {
            tiles: self.tiles.iter()
                .zip(partner.tiles.iter())
                .map(|(a, b)| merge(a, b))
                .collect::<Vec<_>>(),
        }
    }
}

impl Fitness for LinecraftMap {
    fn evaluate(&self) -> FitnessMeasure {
        let base_factor = self.count_friendly_base() as f32 * self.count_enemy_base() as f32;
        let base_distance = self.minimal_distance_between_bases() as f32;
        let empty_value = {
            let count_empty = self.count_empty();
            if count_empty > self.tiles.len() / 4 && count_empty < self.tiles.len() * 3 / 4 {
                2.0
            } else {
                0.25
            }
        };

        let similarity = {
            let value_around_friendly = self.value_around_base(LinecraftTile::FriendlyBase);
            let value_around_enemy = self.value_around_base(LinecraftTile::EnemyBase);

            let min = value_around_friendly.min(value_around_enemy);
            let max = value_around_friendly.max(value_around_enemy);
            min / max
        };

        let resource_value = self.tiles.iter().fold(0.0, |sum, next| {
            sum + next.resource_value()
        });

        if base_factor == 0.0 {
            -resource_value
        } else {
            similarity * base_distance * empty_value * resource_value / (2.0_f32.powf(base_factor))
        }
    }
}